//! The Projects hall, drawn at build time: a garage seen straight down its middle, ceiling
//! lights converging on a far wall (where the walkthrough plays), lane lines on the floor, and
//! one roll-up door per build down the side walls. The page lays its panels over this box by
//! percentage, as sky.rs does for Experience, so drawing and panels stay together at any width.
//! Colours come from CSS classes, so the hall follows day and evening.

pub const W: f64 = 1440.0;
pub const H: f64 = 836.0;
/// The far wall, where the screen hangs.
pub const FAR: (f64, f64, f64, f64) = (400.0, 70.0, 1040.0, 470.0);

/// Where each door sits on its wall: (side, start, end) as fractions of the way from the near
/// edge of the picture to the far wall. Three down the left, four down the right.
const DOORS: [(f64, f64, f64); 7] = [
    (-1.0, 0.08, 0.34), (-1.0, 0.42, 0.62), (-1.0, 0.70, 0.86),
    (1.0, 0.06, 0.28), (1.0, 0.34, 0.52), (1.0, 0.58, 0.73), (1.0, 0.79, 0.90),
];

/// A door's four corners: top near, top far, bottom far, bottom near.
fn door(side: f64, a: f64, b: f64) -> [(f64, f64); 4] {
    let (x0, y0, x1, y1) = FAR;
    let x = |t: f64| if side < 0.0 { x0 * t } else { W - (W - x1) * t };
    let top = |t: f64| 150.0 * (1.0 - t) + (y0 + 40.0) * t;
    let bot = |t: f64| (H - 40.0) * (1.0 - t) + y1 * t;
    [(x(a), top(a)), (x(b), top(b)), (x(b), bot(b)), (x(a), bot(a))]
}

/// The hall. `doors` pairs each door with its build's slug and number, in wall order; `up`
/// is the slug whose door starts raised.
pub fn svg(doors: &[(&str, String)], up: &str) -> String {
    let (x0, y0, x1, y1) = FAR;
    let mut s = format!(
        r#"<svg class="hall-art" viewBox="0 0 {W} {H}" preserveAspectRatio="xMidYMid slice" aria-hidden="true" focusable="false"><defs>
<pattern id="corrugated" width="10" height="9" patternUnits="userSpaceOnUse"><rect class="g-door" width="10" height="9"/><rect class="g-rib" y="7" width="10" height="2"/></pattern>
<radialGradient id="pool" cx="50%" cy="72%" r="40%"><stop offset="0" class="g-pool"/><stop offset="1" class="g-pool g-pool-end"/></radialGradient></defs>
<rect class="g-wall" width="{W}" height="{H}"/>
<polygon class="g-floor" points="0,{H} {W},{H} {x1},{y1} {x0},{y1}"/>
<polygon class="g-ceil" points="0,0 {W},0 {x1},{y0} {x0},{y0}"/>
<rect class="g-far" x="{x0}" y="{y0}" width="{}" height="{}"/>"#,
        x1 - x0,
        y1 - y0
    );
    for t in [0.2, 0.5, 0.8] {
        s += &format!(r#"<line class="g-lane" x1="{}" y1="{y1}" x2="{}" y2="{H}"/>"#, x0 + (x1 - x0) * t, W * t);
    }
    for t in [0.35, 0.75] {
        s += &format!(r#"<line class="g-light" x1="{}" y1="{}" x2="{}" y2="{}" stroke-width="{}"/>"#,
            x0 * (1.0 - t), y0 * (1.0 - t), W - (W - x1) * (1.0 - t), y0 * (1.0 - t), 3.0 + 4.0 * t);
    }
    for (k, (slug, num)) in doors.iter().enumerate().take(DOORS.len()) {
        let (side, a, b) = DOORS[k];
        let p = door(side, a, b);
        let pts = p.iter().map(|(x, y)| format!("{x:.1},{y:.1}")).collect::<Vec<_>>().join(" ");
        let cls = if *slug == up { "g-bay up" } else { "g-bay" };
        let (mx, my) = ((p[0].0 + p[1].0) / 2.0, (p[0].1 + p[1].1) / 2.0 - 8.0);
        let size = 15.0 - 6.0 * (a + b) / 2.0;
        // The opening behind, then the shutter in front; raising the door clips the shutter up.
        s += &format!(r#"<g class="{cls}" data-slug="{slug}"><polygon class="g-open" points="{pts}"/><polygon class="g-shutter" points="{pts}"/><text x="{mx:.1}" y="{my:.1}" font-size="{size:.1}">{num}</text></g>"#);
    }
    s + r#"<rect width="1440" height="836" fill="url(#pool)"/></svg>"#
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every door stays on its own wall: left of the far wall on the left, right of it on the
    /// right, and inside the picture.
    #[test]
    fn doors_stay_on_their_walls() {
        for (side, a, b) in DOORS {
            for (x, y) in door(side, a, b) {
                assert!((0.0..=W).contains(&x) && (0.0..=H).contains(&y));
                if side < 0.0 { assert!(x <= FAR.0) } else { assert!(x >= FAR.2) }
            }
        }
    }
}
