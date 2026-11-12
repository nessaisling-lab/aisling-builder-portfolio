//! The Projects hall, drawn at build time: a garage seen straight down its middle, ceiling
//! lights converging on a far wall (where the walkthrough plays), lane lines on the floor, and
//! one big roll-up door down the left wall. The page lays its panels over this box by
//! percentage, as sky.rs does for Experience, so drawing and panels stay together at any width.
//! Colours come from CSS classes, so the hall follows day and evening. The contact desk in the
//! corner is drawn here too.

pub const W: f64 = 1440.0;
pub const H: f64 = 836.0;
/// The far wall, where the screen hangs. Tall enough to hold the screen and the transcript
/// button, so nothing below the screen spills onto the floor.
pub const FAR: (f64, f64, f64, f64) = (400.0, 60.0, 1040.0, 560.0);

/// A point on the left wall, `t` of the way from the near edge of the picture to the far wall,
/// at the top (`top`) or the bottom of the wall.
fn left_wall(t: f64, top: bool) -> (f64, f64) {
    let (x0, y0, _, y1) = FAR;
    let y = if top { 150.0 * (1.0 - t) + (y0 + 40.0) * t } else { (H - 40.0) * (1.0 - t) + y1 * t };
    (x0 * t, y)
}

/// The hall.
pub fn svg() -> String {
    let (x0, y0, x1, y1) = FAR;
    let mut s = format!(
        r#"<svg class="hall-art" viewBox="0 0 {W} {H}" preserveAspectRatio="xMidYMid slice" aria-hidden="true" focusable="false"><defs>
<pattern id="corrugated" width="12" height="11" patternUnits="userSpaceOnUse"><rect class="g-door" width="12" height="11"/><rect class="g-rib" y="8" width="12" height="3"/></pattern>
<radialGradient id="pool" cx="50%" cy="76%" r="38%"><stop offset="0" class="g-pool"/><stop offset="1" class="g-pool g-pool-end"/></radialGradient></defs>
<rect class="g-wall" width="{W}" height="{H}"/>
<polygon class="g-floor" points="0,{H} {W},{H} {x1},{y1} {x0},{y1}"/>
<polygon class="g-ceil" points="0,0 {W},0 {x1},{y0} {x0},{y0}"/>
<rect class="g-far" x="{x0}" y="{y0}" width="{}" height="{}"/>
<line class="g-skirt" x1="{x0}" y1="{y1}" x2="{x1}" y2="{y1}"/>"#,
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
    // One big roll-up door on the left wall, a hand's width open at the bottom, light under it.
    let (a, b) = (0.30, 0.82);
    let (ta, tb, bb, ba) = (left_wall(a, true), left_wall(b, true), left_wall(b, false), left_wall(a, false));
    let gap = 0.06; // how far open, as a share of the door's height
    let lift = |top: (f64, f64), bot: (f64, f64)| (bot.0, bot.1 - (bot.1 - top.1) * gap);
    let (la, lb) = (lift(ta, ba), lift(tb, bb));
    let pts = |ps: [(f64, f64); 4]| ps.iter().map(|(x, y)| format!("{x:.1},{y:.1}")).collect::<Vec<_>>().join(" ");
    s += &format!(r#"<polygon class="g-frame" points="{}"/>"#, pts([ta, tb, bb, ba]));
    s += &format!(r#"<polygon class="g-gap" points="{}"/>"#, pts([la, lb, bb, ba]));
    s += &format!(r#"<polygon class="g-shutter" points="{}"/>"#, pts([ta, tb, lb, la]));
    s + r#"<rect width="1440" height="836" fill="url(#pool)"/></svg>"#
}

/// The contact desk in the corner: a little computer, a phone and a fax machine on a desk in
/// the desk's clay. The page wraps it in a link to Contact.
pub fn desk() -> &'static str {
    r#"<svg class="desk-art" viewBox="0 0 260 160" aria-hidden="true" focusable="false">
<rect class="d-face" x="20" y="104" width="8" height="52" rx="2"/><rect class="d-face" x="232" y="104" width="8" height="52" rx="2"/>
<rect class="d-top" x="8" y="80" width="244" height="12" rx="2"/><rect class="d-face" x="8" y="92" width="244" height="12"/>
<rect class="d-face" x="152" y="104" width="72" height="24" rx="2"/><rect class="d-pull" x="176" y="114" width="24" height="4" rx="2"/>
<rect class="d-dev" x="22" y="28" width="68" height="50" rx="5"/><rect class="d-screen" x="28" y="34" width="56" height="36" rx="2"/>
<path class="d-glyph" d="M35 46h20M35 52h32M35 58h14"/><rect class="d-dev" x="50" y="78" width="12" height="4"/>
<rect class="d-key" x="24" y="83" width="64" height="5" rx="1.5"/>
<path class="d-dev" d="M110 66h40l6 16h-52z"/><rect class="d-key" x="104" y="58" width="52" height="10" rx="5"/>
<circle class="d-btn" cx="124" cy="74" r="1.6"/><circle class="d-btn" cx="130" cy="74" r="1.6"/><circle class="d-btn" cx="136" cy="74" r="1.6"/>
<path class="d-cord" d="M108 80c-8 4-3 9-8 13s2 9-4 13"/>
<path class="d-paper" d="M186 58h44l-4-16h-36z"/><rect class="d-dev" x="178" y="56" width="62" height="26" rx="3"/>
<rect class="d-screen" x="212" y="62" width="20" height="8" rx="1"/><path class="d-paper" d="M186 82h44l4 10h-52z"/>
</svg>"#
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The door stays on the left wall: left of the far wall and inside the picture.
    #[test]
    fn the_door_stays_on_its_wall() {
        for t in [0.30, 0.82] {
            for top in [true, false] {
                let (x, y) = left_wall(t, top);
                assert!((0.0..=FAR.0).contains(&x) && (0.0..=H).contains(&y));
            }
        }
    }
}
