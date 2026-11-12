//! The home world's colours, as data with their invariants attached.
//!
//! The brand kit (`brand/ness.css`) owns the page colours and checks them in its own build.
//! What the kit doesn't have is this site's world: the warm desk and the city through the
//! window. Those live here. `contrast_x100` is a `const fn`, so a desk colour that makes
//! the kit's text hard to read is a compile error rather than something a person has to
//! notice. Day is light brass and clay; evening is the same desk with the lamp off and the
//! city's windows lit.

/// A colour, kept as three channels so contrast can be computed from it.
#[derive(Clone, Copy, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

// ── compile-time contrast ────────────────────────────────────────────────────
//
// WCAG needs sRGB channels linearised by `((c+0.055)/1.055)^2.4`, and `powf` is not
// available in a const fn. So the curve is precomputed once as fixed-point integers
// (linear value x 1_000_000) and everything downstream is integer arithmetic.

#[rustfmt::skip]
const SRGB_LINEAR: [u32; 256] = [
          0,     304,     607,     911,    1214,    1518,    1821,    2125,
       2428,    2732,    3035,    3347,    3677,    4025,    4391,    4777,
       5182,    5605,    6049,    6512,    6995,    7499,    8023,    8568,
       9134,    9721,   10330,   10960,   11612,   12286,   12983,   13702,
      14444,   15209,   15996,   16807,   17642,   18500,   19382,   20289,
      21219,   22174,   23153,   24158,   25187,   26241,   27321,   28426,
      29557,   30713,   31896,   33105,   34340,   35601,   36889,   38204,
      39546,   40915,   42311,   43735,   45186,   46665,   48172,   49707,
      51269,   52861,   54480,   56128,   57805,   59511,   61246,   63010,
      64803,   66626,   68478,   70360,   72272,   74214,   76185,   78187,
      80220,   82283,   84376,   86500,   88656,   90842,   93059,   95307,
      97587,   99899,  102242,  104616,  107023,  109462,  111932,  114435,
     116971,  119538,  122139,  124772,  127438,  130136,  132868,  135633,
     138432,  141263,  144128,  147027,  149960,  152926,  155926,  158961,
     162029,  165132,  168269,  171441,  174647,  177888,  181164,  184475,
     187821,  191202,  194618,  198069,  201556,  205079,  208637,  212231,
     215861,  219526,  223228,  226966,  230740,  234551,  238398,  242281,
     246201,  250158,  254152,  258183,  262251,  266356,  270498,  274677,
     278894,  283149,  287441,  291771,  296138,  300544,  304987,  309469,
     313989,  318547,  323143,  327778,  332452,  337164,  341914,  346704,
     351533,  356400,  361307,  366253,  371238,  376262,  381326,  386429,
     391572,  396755,  401978,  407240,  412543,  417885,  423268,  428690,
     434154,  439657,  445201,  450786,  456411,  462077,  467784,  473531,
     479320,  485150,  491021,  496933,  502886,  508881,  514918,  520996,
     527115,  533276,  539479,  545724,  552011,  558340,  564712,  571125,
     577580,  584078,  590619,  597202,  603827,  610496,  617207,  623960,
     630757,  637597,  644480,  651406,  658375,  665387,  672443,  679542,
     686685,  693872,  701102,  708376,  715694,  723055,  730461,  737910,
     745404,  752942,  760525,  768151,  775822,  783538,  791298,  799103,
     806952,  814847,  822786,  830770,  838799,  846873,  854993,  863157,
     871367,  879622,  887923,  896269,  904661,  913099,  921582,  930111,
     938686,  947307,  955973,  964686,  973445,  982251,  991102, 1000000,
];

/// Relative luminance, fixed point (x 1_000_000).
const fn luminance(c: Rgb) -> u64 {
    let r = SRGB_LINEAR[c.0 as usize] as u64;
    let g = SRGB_LINEAR[c.1 as usize] as u64;
    let b = SRGB_LINEAR[c.2 as usize] as u64;
    (2126 * r + 7152 * g + 722 * b) / 10_000
}

/// WCAG contrast ratio x 100, so 4.5:1 reads as 450. Integer only, so it const-evaluates.
pub const fn contrast_x100(a: Rgb, b: Rgb) -> u64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    ((hi + 50_000) * 100) / (lo + 50_000)
}

/// Fails the build if `$fg` on `$bg` drops below `$min` x100, naming the pairing.
macro_rules! assert_readable {
    ($fg:expr, $bg:expr, $min:expr, $what:literal) => {
        const _: () = assert!(
            contrast_x100($fg, $bg) >= $min,
            concat!("palette contrast below WCAG minimum: ", $what),
        );
    };
}

// ── the worlds ───────────────────────────────────────────────────────────────

pub struct World {
    pub name: &'static str,
    /// The kit's text colours, mirrored from brand/ness.css (oklch converted to sRGB) so
    /// they can be checked against the desk. If the kit's ink changes, change it here too.
    pub ink: Rgb,
    pub ink_2: Rgb,
    pub muted: Rgb,
    pub accent: Rgb,
    /// Drawer fronts: the desk surface that carries text.
    pub desk_face: Rgb,
    /// The desk top, and the clay tablet that sits on it.
    pub desk_top: Rgb,
    /// The desk's front panel around the drawers. Decorative.
    pub desk_front: Rgb,
    /// Edges and incised lines. Decorative.
    pub desk_cut: Rgb,
    /// The city through the window. Decorative: no text sits on it.
    pub sky: Rgb,
    pub city: Rgb,
    pub city_2: Rgb,
    /// Window panes on the buildings: dark glass by day, lamplight in the evening.
    pub lit: Rgb,
    /// Panes nobody is home behind. By day every pane is the same glass.
    pub unlit: Rgb,
    /// The Experience sky chart, top and horizon. Text sits anywhere on it.
    pub chart_top: Rgb,
    pub chart_bottom: Rgb,
    /// Marks on that sky: the stars where jobs cross the meridian, and their dates.
    pub mark: Rgb,
    /// The strip of earth under the sky, which carries how I work.
    pub earth: Rgb,
}

/// Day: light brass and clay tints (Aisling's choice, Nov 11: warm fits Sumer), the kit's
/// cool petrol for the city.
pub const DAY: World = World {
    name: "day",
    ink: Rgb(0x10, 0x1D, 0x22),
    ink_2: Rgb(0x32, 0x40, 0x45),
    muted: Rgb(0x50, 0x5D, 0x63),
    accent: Rgb(0x0C, 0x60, 0x6D),
    desk_face: Rgb(0xF4, 0xEA, 0xD5),
    desk_top: Rgb(0xEA, 0xD9, 0xB6),
    desk_front: Rgb(0xE1, 0xCB, 0xA2),
    desk_cut: Rgb(0xB7, 0x99, 0x6C),
    sky: Rgb(0xCA, 0xE3, 0xED),
    city: Rgb(0x85, 0x9D, 0xA6),
    city_2: Rgb(0x5F, 0x79, 0x84),
    lit: Rgb(0xA9, 0xC4, 0xCF),
    unlit: Rgb(0xA9, 0xC4, 0xCF),
    // dawn: pale sky warming to the horizon, petrol marks
    chart_top: Rgb(0xDC, 0xEB, 0xF1),
    chart_bottom: Rgb(0xF2, 0xE9, 0xD8),
    mark: Rgb(0x0C, 0x60, 0x6D),
    earth: Rgb(0xE1, 0xCB, 0xA2),
};

/// Evening: the same desk in low light; the windows across the street come on.
pub const EVENING: World = World {
    name: "evening",
    ink: Rgb(0xE9, 0xF0, 0xF0),
    ink_2: Rgb(0xC8, 0xD3, 0xD4),
    muted: Rgb(0xA4, 0xB1, 0xB1),
    accent: Rgb(0x65, 0xD0, 0xDC),
    desk_face: Rgb(0x3B, 0x31, 0x21),
    desk_top: Rgb(0x31, 0x27, 0x18),
    desk_front: Rgb(0x27, 0x1D, 0x10),
    desk_cut: Rgb(0x62, 0x53, 0x3C),
    sky: Rgb(0x1B, 0x2A, 0x3C),
    city: Rgb(0x0E, 0x1C, 0x28),
    city_2: Rgb(0x08, 0x13, 0x1C),
    lit: Rgb(0xE6, 0xD8, 0xA3),
    unlit: Rgb(0x16, 0x26, 0x33),
    // night: the stars come out in lamplight
    chart_top: Rgb(0x08, 0x12, 0x1B),
    chart_bottom: Rgb(0x1F, 0x30, 0x45),
    mark: Rgb(0xE6, 0xD8, 0xA3),
    earth: Rgb(0x1A, 0x14, 0x0C),
};

// ── the invariants ───────────────────────────────────────────────────────────
//
// Text sits on the drawer fronts (names, one-line summaries, Sumerian words) and on the
// clay tablet. Every kit text colour that can land there is held to 4.5:1; the small
// lines get no large-text discount.

assert_readable!(DAY.ink, DAY.desk_face, 450, "day ink on drawer");
assert_readable!(DAY.ink_2, DAY.desk_face, 450, "day ink-2 on drawer");
assert_readable!(DAY.muted, DAY.desk_face, 450, "day muted on drawer");
assert_readable!(DAY.accent, DAY.desk_face, 450, "day accent on drawer");
assert_readable!(DAY.ink_2, DAY.desk_top, 450, "day ink-2 on tablet");

assert_readable!(EVENING.ink, EVENING.desk_face, 450, "evening ink on drawer");
assert_readable!(EVENING.ink_2, EVENING.desk_face, 450, "evening ink-2 on drawer");
assert_readable!(EVENING.muted, EVENING.desk_face, 450, "evening muted on drawer");
assert_readable!(EVENING.accent, EVENING.desk_face, 450, "evening accent on drawer");
assert_readable!(EVENING.ink_2, EVENING.desk_top, 450, "evening ink-2 on tablet");

// The Experience sky: job names, roles, dates and facts sit anywhere from the top of the chart
// to the horizon; how I work sits on the earth.
assert_readable!(DAY.ink, DAY.chart_top, 450, "day ink on sky");
assert_readable!(DAY.ink_2, DAY.chart_bottom, 450, "day ink-2 on horizon");
assert_readable!(DAY.muted, DAY.chart_bottom, 450, "day muted on horizon");
assert_readable!(DAY.mark, DAY.chart_top, 450, "day mark on sky");
assert_readable!(DAY.mark, DAY.chart_bottom, 450, "day mark on horizon");
assert_readable!(DAY.mark, DAY.earth, 450, "day mark on earth");
assert_readable!(DAY.ink_2, DAY.earth, 450, "day ink-2 on earth");
assert_readable!(EVENING.ink, EVENING.chart_bottom, 450, "evening ink on sky");
assert_readable!(EVENING.ink_2, EVENING.chart_bottom, 450, "evening ink-2 on horizon");
assert_readable!(EVENING.muted, EVENING.chart_bottom, 450, "evening muted on horizon");
assert_readable!(EVENING.mark, EVENING.chart_bottom, 450, "evening mark on horizon");
assert_readable!(EVENING.mark, EVENING.earth, 450, "evening mark on earth");
assert_readable!(EVENING.ink_2, EVENING.earth, 450, "evening ink-2 on earth");
assert_readable!(DAY.accent, DAY.earth, 450, "day link on earth");
assert_readable!(EVENING.accent, EVENING.earth, 450, "evening link on earth");

/// The CSS custom properties for one world. Contrast figures in the comments are
/// computed from the same values the assertions check, so they cannot go stale.
pub fn css_vars(w: &World) -> String {
    let r = |fg: Rgb| contrast_x100(fg, w.desk_face) as f64 / 100.0;
    format!(
        "  /* {name} */\n\
         \x20 --desk-face:{face};   /* ink {ri:.2}:1, muted {rm:.2}:1 */\n\
         \x20 --desk-top:{top};--desk-front:{front};--desk-cut:{cut};\n\
         \x20 --sky:{sky};--city:{city};--city-2:{city2};--lit:{lit};--unlit:{unlit};\n\
         \x20 --chart-top:{ct};--chart-bottom:{cb};--mark:{mark};--earth:{earth};\n",
        name = w.name,
        face = w.desk_face.hex(),
        top = w.desk_top.hex(),
        front = w.desk_front.hex(),
        cut = w.desk_cut.hex(),
        sky = w.sky.hex(),
        city = w.city.hex(),
        city2 = w.city_2.hex(),
        lit = w.lit.hex(),
        unlit = w.unlit.hex(),
        ct = w.chart_top.hex(),
        cb = w.chart_bottom.hex(),
        mark = w.mark.hex(),
        earth = w.earth.hex(),
        ri = r(w.ink),
        rm = r(w.muted),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Anchors the fixed-point maths: black on white is exactly 21:1, anything on
    /// itself exactly 1:1.
    #[test]
    fn known_ratios() {
        let black = Rgb(0, 0, 0);
        let white = Rgb(255, 255, 255);
        assert_eq!(contrast_x100(black, white), 2100);
        assert_eq!(contrast_x100(white, white), 100);
    }

    /// Lit windows are the evening's one event. By day the panes have to stay close to
    /// the building, or the skyline reads as lit at noon.
    #[test]
    fn windows_light_up_only_in_the_evening() {
        assert!(contrast_x100(DAY.lit, DAY.city) < 160, "day windows look lit");
        assert!(contrast_x100(EVENING.lit, EVENING.city) > 700, "evening windows look dark");
    }
}
