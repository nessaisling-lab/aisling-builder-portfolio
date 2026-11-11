//! The city through the home page's window, drawn at build time as inline SVG.
//!
//! The buildings are a fixed list, not random, so every render is the same page. Which
//! panes are lit comes from a small hash of each pane's position. CSS does the colouring
//! (theme.rs): by day every pane is the same glass; in the evening the lit ones come on.

struct Building {
    x: u32,
    w: u32,
    h: u32,
    /// Set back a block: drawn darker, behind the street front.
    back: bool,
    /// Rounded crown, for the one tower that isn't a box.
    round: bool,
    /// A wooden water tank on the roof, the New York kind.
    tank: bool,
}

const fn b(x: u32, w: u32, h: u32, back: bool) -> Building {
    Building { x, w, h, back, round: false, tank: false }
}

const CITY: &[Building] = &[
    b(0, 92, 150, false),
    b(86, 60, 232, true),
    Building { tank: true, ..b(152, 118, 122, false) },
    Building { round: true, ..b(286, 46, 262, true) },
    Building { tank: true, ..b(378, 132, 142, false) },
    b(520, 82, 204, true),
    b(610, 110, 112, false),
];

pub const W: u32 = 720;
pub const H: u32 = 290;

/// Roughly two panes in seven are lit. Deterministic, so the page never changes on its own.
fn lit(x: u32, y: u32) -> bool {
    (x.wrapping_mul(73_856_093) ^ y.wrapping_mul(19_349_663)) % 7 < 2
}

pub fn svg() -> String {
    let mut s = format!(
        "<svg class=\"skyline\" viewBox=\"0 0 {W} {H}\" preserveAspectRatio=\"xMidYMax slice\" \
         aria-hidden=\"true\" focusable=\"false\">"
    );
    // Back row first, so the street front overlaps it.
    for back in [true, false] {
        for c in CITY.iter().filter(|c| c.back == back) {
            let (x, w, top) = (c.x, c.w, H - c.h);
            let cls = if c.back { "bk" } else { "fr" };
            if c.round {
                let r = w / 2;
                s += &format!(
                    "<path class=\"{cls}\" d=\"M{x} {H}V{y}a{r} {r} 0 0 1 {w} 0V{H}Z\"/>",
                    y = top + r
                );
            } else {
                s += &format!("<rect class=\"{cls}\" x=\"{x}\" y=\"{top}\" width=\"{w}\" height=\"{h}\"/>", h = c.h);
            }
            let first_row = if c.round { top + w / 2 + 6 } else { top + 14 };
            for py in (first_row..H - 12).step_by(18) {
                for px in (x + 9..x + w - 12).step_by(14) {
                    let on = if lit(px, py) { " on" } else { "" };
                    s += &format!("<rect class=\"pn{on}\" x=\"{px}\" y=\"{py}\" width=\"6\" height=\"9\"/>");
                }
            }
            if c.tank {
                // Legs, tank, conical roof: 40 wide, standing on the roofline.
                let tx = x + w / 3;
                s += &format!(
                    "<g class=\"tank\"><rect x=\"{l1}\" y=\"{ly}\" width=\"3\" height=\"16\"/>\
                     <rect x=\"{l2}\" y=\"{ly}\" width=\"3\" height=\"16\"/>\
                     <rect x=\"{tx}\" y=\"{ty}\" width=\"40\" height=\"32\" rx=\"4\"/>\
                     <path d=\"M{tx} {ty}L{mid} {cy}L{rx} {ty}Z\"/></g>",
                    l1 = tx + 6,
                    l2 = tx + 31,
                    ly = top - 16,
                    ty = top - 48,
                    mid = tx + 20,
                    cy = top - 62,
                    rx = tx + 40,
                );
            }
        }
    }
    s + "</svg>"
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every building has to fit the window, or it is cut off on the right.
    #[test]
    fn buildings_fit_the_window() {
        for c in CITY {
            assert!(c.x + c.w <= W && c.h < H, "building at x={} spills out of the window", c.x);
        }
    }

    /// Some windows lit, most dark: an evening, not an office at noon or a blackout.
    #[test]
    fn some_windows_are_lit() {
        let svg = svg();
        let (all, on) = (svg.matches("class=\"pn").count(), svg.matches("pn on").count());
        assert!(all > 100, "only {all} panes");
        assert!(on * 10 > all && on * 2 < all, "{on} of {all} panes lit");
    }
}
