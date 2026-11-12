//! The Projects hall, drawn at build time from one camera: a garage seen straight down its
//! middle at the screen that fills its far wall. Light panels on the ceiling, a tool wall near
//! on the right, one big roll-up door on the left, and the turntable's bay marked on the floor.
//! Everything is a point in the room, in metres, put in the picture by `at`, so every line meets
//! where it should; turntable.js draws the 3D props (the contact desk, the tool chest) from the
//! same camera, read from `room()`. The page lays its panels over this box by percentage, as
//! sky.rs does for Experience. Colours come from CSS classes, so the hall follows day and evening.

pub const W: f64 = 1440.0;
pub const H: f64 = 836.0;
/// The camera: its focal length in pixels, and the vanishing point it looks straight at.
const F: f64 = 1000.0;
const VP: (f64, f64) = (720.0, 340.0);
/// The room, in metres: half its width, the eye's height, the ceiling's, the far wall's distance.
const HALF: f64 = 4.0;
const EYE: f64 = 2.75;
const CEIL: f64 = 5.5;
const BACK: f64 = 10.0;
/// The nearest depth drawn: in front of the picture, so every surface runs off its edges.
const NEAR: f64 = 4.0;
/// The far wall as the camera sees it, where the screen hangs (the tests hold it to `at`).
pub const FAR: (f64, f64, f64, f64) = (320.0, 65.0, 1120.0, 615.0);

/// Where a point in the room lands in the picture: `x` across from the middle, `y` up from the
/// floor, `z` away from the camera.
fn at(x: f64, y: f64, z: f64) -> (f64, f64) {
    (VP.0 + F * x / z, VP.1 - F * (y - EYE) / z)
}

fn pts(ps: &[(f64, f64, f64)]) -> String {
    ps.iter().map(|&(x, y, z)| { let (a, b) = at(x, y, z); format!("{a:.1},{b:.1}") }).collect::<Vec<_>>().join(" ")
}

fn poly(class: &str, ps: &[(f64, f64, f64)]) -> String {
    format!(r#"<polygon class="{class}" points="{}"/>"#, pts(ps))
}

fn line(class: &str, a: (f64, f64, f64), b: (f64, f64, f64)) -> String {
    let ((x1, y1), (x2, y2)) = (at(a.0, a.1, a.2), at(b.0, b.1, b.2));
    format!(r#"<line class="{class}" x1="{x1:.1}" y1="{y1:.1}" x2="{x2:.1}" y2="{y2:.1}"/>"#)
}

/// The floor's perspective at (x, z), for something flat lying there drawn in its own metres:
/// u across, v toward the camera. Close enough over a metre or two.
fn on_floor(x: f64, z: f64) -> String {
    let (px, py) = at(x, 0.0, z);
    format!("matrix({:.2},0,{:.2},{:.2},{px:.1},{py:.1})", F / z, F * x / (z * z), F * EYE / (z * z))
}

/// A shape on the right wall, drawn in the wall's own metres from (z0, y0): u away from the
/// camera, v up.
fn on_right_wall(class: &str, z0: f64, y0: f64, uv: &[(f64, f64)]) -> String {
    let ps: Vec<_> = uv.iter().map(|&(u, v)| (HALF, y0 + v, z0 + u)).collect();
    poly(class, &ps)
}

/// The tool wall: a pegboard on the right wall, from `PEG.0` to `PEG.1` deep and `PEG.2` to
/// `PEG.3` high, with each tool's painted outline behind it, the way a shop keeps count.
const PEG: (f64, f64, f64, f64) = (6.0, 8.1, 0.85, 1.95);

fn tool_wall() -> String {
    let (z0, z1, y0, y1) = PEG;
    let mut s = poly("g-peg", &[(HALF, y0, z0), (HALF, y1, z0), (HALF, y1, z1), (HALF, y0, z1)]);
    // The holes, one stroke of round dots.
    let mut d = String::new();
    let mut z = z0 + 0.1;
    while z < z1 - 0.05 {
        let mut y = y0 + 0.1;
        while y < y1 - 0.05 {
            let (a, b) = at(HALF, y, z);
            d += &format!("M{a:.1} {b:.1}h0");
            y += 0.15;
        }
        z += 0.15;
    }
    s += &format!(r#"<path class="g-holes" d="{d}"/>"#);
    // Tools, in wall metres: a wrench, a hammer, three screwdrivers, pliers.
    type Outline = &'static [(f64, f64)];
    let tools: [(f64, f64, Outline); 6] = [
        (0.25, 0.2, &[(0.0, 0.0), (0.05, 0.0), (0.05, 0.62), (0.1, 0.66), (0.1, 0.74), (0.06, 0.7), (-0.01, 0.7), (-0.05, 0.74), (-0.05, 0.66), (0.0, 0.62)]),
        (0.6, 0.2, &[(0.0, 0.0), (0.05, 0.0), (0.05, 0.55), (0.13, 0.55), (0.13, 0.66), (-0.1, 0.66), (-0.1, 0.58), (0.0, 0.55)]),
        (0.95, 0.3, &[(0.0, 0.0), (0.015, 0.0), (0.015, 0.3), (0.035, 0.3), (0.035, 0.5), (-0.02, 0.5), (-0.02, 0.3), (0.0, 0.3)]),
        (1.12, 0.35, &[(0.0, 0.0), (0.015, 0.0), (0.015, 0.25), (0.035, 0.25), (0.035, 0.45), (-0.02, 0.45), (-0.02, 0.25), (0.0, 0.25)]),
        (1.29, 0.3, &[(0.0, 0.0), (0.015, 0.0), (0.015, 0.34), (0.035, 0.34), (0.035, 0.52), (-0.02, 0.52), (-0.02, 0.34), (0.0, 0.34)]),
        (1.55, 0.25, &[(0.0, 0.0), (0.04, 0.0), (0.09, 0.4), (0.06, 0.62), (0.03, 0.62), (0.05, 0.4), (0.02, 0.18), (-0.01, 0.4), (0.01, 0.62), (-0.02, 0.62), (-0.05, 0.4)]),
    ];
    for (u, v, shape) in tools {
        let outline: Vec<_> = shape.iter().map(|&(a, b)| (a - 0.02, b + 0.02)).collect();
        s += &on_right_wall("g-tool-ghost", z0 + u, y0 + v, &outline);
        s += &on_right_wall("g-tool", z0 + u, y0 + v, shape);
    }
    s
}

/// The hall.
pub fn svg() -> String {
    let (x0, y0, x1, y1) = FAR;
    let mut s = format!(
        r#"<svg class="hall-art" viewBox="0 0 {W} {H}" preserveAspectRatio="xMidYMid slice" aria-hidden="true" focusable="false"><defs>
<pattern id="hazard" width="22" height="22" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><rect class="g-haz" width="11" height="22"/></pattern>
<radialGradient id="pool" cx="50%" cy="76%" r="38%"><stop offset="0" class="g-pool"/><stop offset="1" class="g-pool g-pool-end"/></radialGradient>
<radialGradient id="lamp-pool"><stop offset="0" class="g-pool"/><stop offset="1" class="g-pool g-pool-end"/></radialGradient>
<linearGradient id="sheen" x1="0" y1="0" x2="0" y2="1"><stop offset="0" class="g-sheen"/><stop offset="1" class="g-sheen g-sheen-end"/></linearGradient>
<filter id="glow" x="-40%" y="-40%" width="180%" height="180%"><feGaussianBlur stdDeviation="7"/></filter></defs>
<rect class="g-wall" width="{W}" height="{H}"/>"#
    );
    // The walls: a darker band to waist height, a kick plate, panel seams.
    for x in [-HALF, HALF] {
        s += &poly("g-wains", &[(x, 0.0, NEAR), (x, 1.1, NEAR), (x, 1.1, BACK), (x, 0.0, BACK)]);
        s += &poly("g-kick", &[(x, 0.0, NEAR), (x, 0.16, NEAR), (x, 0.16, BACK), (x, 0.0, BACK)]);
        for z in [5.0, 6.25, 7.5, 8.75] {
            s += &line("g-seam", (x, 0.0, z), (x, CEIL, z));
        }
    }
    // The ceiling, its beams, and two light panels with their glow.
    s += &poly("g-ceil", &[(-HALF, CEIL, NEAR), (HALF, CEIL, NEAR), (HALF, CEIL, BACK), (-HALF, CEIL, BACK)]);
    for z in [6.0, 8.0] {
        s += &line("g-beam", (-HALF, CEIL, z), (HALF, CEIL, z));
    }
    const LAMPS: [(f64, f64); 2] = [(6.4, 7.0), (8.3, 8.8)];
    let lamps: String = LAMPS.iter().map(|&(a, b)| poly("g-lamp", &[(-2.2, CEIL, a), (2.2, CEIL, a), (2.2, CEIL, b), (-2.2, CEIL, b)])).collect();
    s += &format!(r#"<g class="g-lamp-glow" filter="url(#glow)">{lamps}</g>{lamps}"#);
    // The far wall, which the screen fills.
    s += &format!(r#"<rect class="g-far" x="{x0}" y="{y0}" width="{}" height="{}"/>"#, x1 - x0, y1 - y0);
    s += &poly("g-kick", &[(-HALF, 0.0, BACK), (HALF, 0.0, BACK), (HALF, 0.16, BACK), (-HALF, 0.16, BACK)]);
    // The floor: satin, lit from the lamps, its slab joints and the bay's lane lines.
    let floor = [(-HALF, 0.0, NEAR), (HALF, 0.0, NEAR), (HALF, 0.0, BACK), (-HALF, 0.0, BACK)];
    s += &poly("g-floor", &floor);
    s += &poly("g-sheen-fill", &floor);
    for &(a, b) in &LAMPS {
        s += &format!(r#"<ellipse class="g-lamp-pool" rx="2.6" ry="1.1" transform="{}"/>"#, on_floor(0.0, (a + b) / 2.0));
    }
    for x in [-HALF / 1.5, HALF / 1.5] {
        s += &line("g-joint", (x, 0.0, NEAR), (x, 0.0, BACK));
    }
    for z in [7.5, 10.0] {
        s += &line("g-joint", (-HALF, 0.0, z), (HALF, 0.0, z));
    }
    for x in [-2.45, 2.45] {
        s += &line("g-lane", (x, 0.0, NEAR), (x, 0.0, BACK));
    }
    s += &format!(r#"<line class="g-skirt" x1="{x0}" y1="{y1}" x2="{x1}" y2="{y1}"/>"#);
    // Oil, where cars have stood.
    for (x, z, r) in [(-1.0, 9.4, 0.5), (2.9, 9.2, 0.3), (-3.0, 8.6, 0.45)] {
        s += &format!(r#"<ellipse class="g-stain" rx="{r}" ry="{:.2}" transform="{}"/>"#, r * 0.6, on_floor(x, z));
    }
    // The turntable's bay, edged in hazard stripes, and the bay's number stencilled on the far
    // wall under the screen (site.js changes it with the build on show).
    let (bx, bz0, bz1, w) = (2.1, 5.3, 7.7, 0.2);
    s += &format!(
        r#"<path class="g-bay-edge" fill-rule="evenodd" d="M{}Z M{}Z"/>"#,
        pts(&[(-bx, 0.0, bz0), (bx, 0.0, bz0), (bx, 0.0, bz1), (-bx, 0.0, bz1)]).replace(' ', " L"),
        pts(&[(-bx + w, 0.0, bz0 + w), (bx - w, 0.0, bz0 + w), (bx - w, 0.0, bz1 - w), (-bx + w, 0.0, bz1 - w)]).replace(' ', " L"),
    );
    s += &format!(r#"<text class="g-bay" id="bay-n" x="{}" y="{}">BAY 06</text>"#, x0 + 22.0, y1 - 22.0);
    // One big roll-up door on the left wall, behind the list of builds, a hand's width open at
    // the bottom with light under it.
    // Its slats run along the wall, so they close on the vanishing point like every line here;
    // guide rails either side, the roller's housing above, a bottom bar with a pull.
    let (da, db, dh, gap) = (6.0, 9.3, 4.0, 0.26);
    s += &poly("g-frame", &[(-HALF, 0.0, da - 0.2), (-HALF, dh + 0.55, da - 0.2), (-HALF, dh + 0.55, db + 0.2), (-HALF, 0.0, db + 0.2)]);
    s += &poly("g-gap", &[(-HALF, 0.0, da), (-HALF, gap, da), (-HALF, gap, db), (-HALF, 0.0, db)]);
    s += &poly("g-shutter", &[(-HALF, gap, da), (-HALF, dh, da), (-HALF, dh, db), (-HALF, gap, db)]);
    let mut y = gap + 0.14;
    while y < dh - 0.05 {
        s += &line("g-slat", (-HALF, y, da), (-HALF, y, db));
        y += 0.14;
    }
    for z in [da, db] {
        s += &line("g-rail", (-HALF, 0.0, z), (-HALF, dh, z));
    }
    s += &poly("g-roller", &[(-HALF, dh, da - 0.2), (-HALF, dh + 0.55, da - 0.2), (-HALF, dh + 0.55, db + 0.2), (-HALF, dh, db + 0.2)]);
    s += &line("g-bar", (-HALF, gap + 0.04, da), (-HALF, gap + 0.04, db));
    let m = (da + db) / 2.0;
    s += &poly("g-pull", &[(-HALF, gap + 0.1, m - 0.25), (-HALF, gap + 0.2, m - 0.25), (-HALF, gap + 0.2, m + 0.25), (-HALF, gap + 0.1, m + 0.25)]);
    s += &tool_wall();
    s + r#"<rect width="1440" height="836" fill="url(#pool)"/></svg>"#
}

/// The camera and the room, for turntable.js: the picture's size, focal length, vanishing
/// point, half width, eye height, ceiling height, far wall.
pub fn room() -> String {
    format!("{W} {H} {F} {} {} {HALF} {EYE} {CEIL} {BACK}", VP.0, VP.1)
}

/// The props as JSON for turntable.js, each with its Library file's URL under `root`.
pub fn props_json(root: &str) -> String {
    let items: Vec<String> = PROPS.iter()
        .map(|p| format!(r#"{{"url":"{root}brand/library/{}-model.json","x":{},"z":{},"turn":{}}}"#, p.file, p.x, p.z, p.turn))
        .collect();
    format!("[{}]", items.join(","))
}

/// A 3D prop standing in the room, from the brand kit's Library: its file, where it stands
/// (metres across and away), and how far it's turned (radians, anticlockwise from above).
pub struct Prop {
    pub file: &'static str,
    pub x: f64,
    pub z: f64,
    pub turn: f64,
}

/// The contact desk (its origin is its front left corner) under the screen's bottom right
/// corner, in front of the tool wall, and the tool chest nearer, under the wall's near end.
pub const PROPS: &[Prop] = &[
    Prop { file: "built--contact-desk", x: 2.4, z: 7.2, turn: 0.0 },
    Prop { file: "built--tool-chest", x: 3.45, z: 5.9, turn: -1.3 },
];

/// The contact desk's box in the room: 1.43 m wide, 0.76 deep, 1.32 high to the monitor's top.
const DESK: (f64, f64, f64) = (1.43, 0.76, 1.32);

/// Where the contact desk's link sits over the picture, as CSS percentages of the hall, and
/// where its note sits on the monitor, as percentages of that link. Custom properties, so the
/// narrow layout, which stacks everything, can ignore them.
pub fn desk_spot() -> (String, String) {
    let p = &PROPS[0];
    let (mut l, mut t, mut r, mut b) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for x in [p.x, p.x + DESK.0] {
        for y in [0.0, DESK.2] {
            for z in [p.z, p.z + DESK.1] {
                let (a, c) = at(x, y, z);
                (l, t, r, b) = (l.min(a), t.min(c), r.max(a), b.max(c));
            }
        }
    }
    let (nx, ny) = at(p.x + 0.86, 1.22, p.z + 0.5);
    (
        format!("--l:{:.2}%;--t:{:.2}%;--w:{:.2}%;--h:{:.2}%", l / W * 100.0, t / H * 100.0, (r - l) / W * 100.0, (b - t) / H * 100.0),
        format!("--nx:{:.1}%;--ny:{:.1}%", (nx - l) / (r - l) * 100.0, (ny - t) / (b - t) * 100.0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The camera puts the far wall's corners where the screen's place on the page expects them.
    #[test]
    fn the_far_wall_is_where_the_screen_hangs() {
        let near = |a: (f64, f64), b: (f64, f64)| (a.0 - b.0).abs() < 0.5 && (a.1 - b.1).abs() < 0.5;
        assert!(near(at(-HALF, CEIL, BACK), (FAR.0, FAR.1)));
        assert!(near(at(HALF, 0.0, BACK), (FAR.2, FAR.3)));
    }

    /// Every prop stands inside the room, clear of the turntable's bay and of each other.
    #[test]
    fn props_stand_in_the_room() {
        for p in PROPS {
            assert!(p.x.abs() < HALF && p.z > NEAR && p.z < BACK, "{} is outside the room", p.file);
            assert!(p.x > 2.25 || p.x < -2.25, "{} stands in the turntable's bay", p.file);
        }
        let d = &PROPS[0];
        assert!(PROPS[1..].iter().all(|p| (p.z - d.z).abs() > DESK.1 + 0.4), "the desk is crowded");
        assert!(d.x + DESK.0 < HALF, "the desk runs into the wall");
    }

    /// The desk's link lands in the right half of the hall, below the screen, inside the picture.
    #[test]
    fn the_desk_link_is_on_the_desk() {
        let (spot, note) = desk_spot();
        let n = |s: &str, k: &str| s.split(';').find_map(|p| p.strip_prefix(k)?.strip_suffix('%')?.parse::<f64>().ok()).unwrap();
        let (l, t, w, h) = (n(&spot, "--l:"), n(&spot, "--t:"), n(&spot, "--w:"), n(&spot, "--h:"));
        assert!(l > 60.0 && l + w < 100.0 && t > 50.0 && t + h < 100.0, "{spot}");
        assert!((0.0..100.0).contains(&n(&note, "--nx:")) && (0.0..100.0).contains(&n(&note, "--ny:")), "{note}");
    }
}
