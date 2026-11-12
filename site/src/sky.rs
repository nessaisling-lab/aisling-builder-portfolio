//! The Experience sky chart, drawn at build time.
//!
//! Each job is a star's track across the sky: a curve from one horizon to the other whose
//! highest point sits on the meridian, newest highest. Facts from the job ride along its
//! track. Everything here is in design pixels on a 1440 x 740 chart; the page places its
//! HTML labels by percentage of that box, so the labels and this drawing always line up.

pub const W: f64 = 1440.0;
pub const H: f64 = 740.0;
/// The meridian, the line the tracks peak on.
pub const MX: f64 = 520.0;
/// How far each track runs either side of the meridian before it meets the horizon.
const REACH: f64 = 1150.0;

/// Where job `i` crosses the meridian, from the top of the chart.
pub fn crossing(i: usize) -> f64 {
    126.0 + 133.0 * i as f64
}

/// Where along a job's track each of its facts sits: 0 is the far-left horizon, 1 the far
/// right. Tuned by eye so no fact lands on a job's label or behind the ziggurat; a job past
/// this table spreads its facts evenly down the left of its track.
const ALONG: [&[f64]; 4] = [&[0.33, 0.66, 0.72, 0.79], &[0.30, 0.66, 0.73], &[0.33, 0.40, 0.64], &[0.33, 0.62]];

/// A point on job `i`'s track, `t` of the way along it.
fn on_track(i: usize, t: f64) -> (f64, f64) {
    let (x0, x2) = (MX - REACH, MX + REACH);
    let c = 2.0 * (crossing(i) - H / 2.0); // the control point that puts the peak on `crossing`
    let q = |a: f64, b: f64, d: f64| (1.0 - t).powi(2) * a + 2.0 * t * (1.0 - t) * b + t * t * d;
    (q(x0, MX, x2), q(H, c, H))
}

/// Where fact `k` of job `i` sits, as percentages of the chart.
pub fn fact_at(i: usize, k: usize) -> (f64, f64) {
    let t = ALONG.get(i).and_then(|a| a.get(k)).copied().unwrap_or(0.30 + 0.06 * k as f64);
    let (x, y) = on_track(i, t);
    (x / W * 100.0, y / H * 100.0)
}

/// The drawing behind the labels: stars, the meridian, and one dashed track per job.
pub fn svg(jobs: usize) -> String {
    let mut s = format!(r#"<svg class="sky-art" viewBox="0 0 {W} {H}" aria-hidden="true" focusable="false"><g class="stars">"#);
    // ponytail: a fixed scatter, not random, so the build is reproducible and stars never jump.
    for i in 0..140u64 {
        let x = (i * 7919) % W as u64;
        let mut y = ((i * 104_729) % (H as u64 - 10)) as f64;
        if i % 3 == 0 {
            y *= 0.55; // more stars toward the top of the sky
        }
        let r = if i % 5 == 0 { 1.4 } else { 0.8 };
        s += &format!(r#"<circle cx="{x}" cy="{y:.0}" r="{r}" opacity="{:.2}"/>"#, 0.3 + (i % 7) as f64 / 10.0);
    }
    s += &format!(r#"</g><line class="meridian" x1="{MX}" y1="0" x2="{MX}" y2="{H}"/>"#);
    for i in 0..jobs {
        let c = 2.0 * (crossing(i) - H / 2.0);
        s += &format!(r#"<path class="track" d="M{} {H} Q{MX} {c} {} {H}"/>"#, MX - REACH, MX + REACH);
    }
    s + "</svg>"
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A track peaks exactly on its crossing, so the star drawn there sits on the curve.
    #[test]
    fn tracks_peak_on_the_meridian() {
        for i in 0..4 {
            let (x, y) = on_track(i, 0.5);
            assert!((x - MX).abs() < 1e-9 && (y - crossing(i)).abs() < 1e-9);
        }
    }

    /// The title and deck take the top-left of the sky (to 29% across, about 25% down at four
    /// lines of deck); a fact there would read as part of the deck.
    #[test]
    fn facts_clear_the_title() {
        for (i, along) in ALONG.iter().enumerate() {
            for k in 0..along.len() {
                let (x, y) = fact_at(i, k);
                assert!(x > 30.0 || y > 26.0, "job {i} fact {k} at {x:.1}%, {y:.1}% sits under the deck");
            }
        }
    }

    /// Every fact lands inside the chart, with room for its label to the right.
    #[test]
    fn facts_stay_on_the_chart() {
        for (i, along) in ALONG.iter().enumerate() {
            for k in 0..along.len() {
                let (x, y) = fact_at(i, k);
                assert!((2.0..86.0).contains(&x) && (2.0..98.0).contains(&y), "job {i} fact {k} at {x:.1}%, {y:.1}%");
            }
        }
    }
}
