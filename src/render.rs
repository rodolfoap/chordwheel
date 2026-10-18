//! SVG drawing of the two wheels and the staff. Colours come from CSS classes
//! so the page can theme them.

use std::fmt::Write;

use crate::glyphs;
use crate::state::State;
use crate::theory::*;

pub const R_OUTER: f64 = 196.0;
pub const R_SPLIT: f64 = 148.0;
pub const R_INNER: f64 = 143.0;
pub const R_HUB: f64 = 62.0;

/// Point at `r` and `deg` degrees clockwise from the top.
fn polar(r: f64, deg: f64) -> (f64, f64) {
    let a = deg.to_radians();
    (r * a.sin(), -r * a.cos())
}

fn wedge(r0: f64, r1: f64, a0: f64, a1: f64) -> String {
    let (x0, y0) = polar(r1, a0);
    let (x1, y1) = polar(r1, a1);
    let (x2, y2) = polar(r0, a1);
    let (x3, y3) = polar(r0, a0);
    format!(
        "M{x0:.2} {y0:.2}A{r1} {r1} 0 0 1 {x1:.2} {y1:.2}L{x2:.2} {y2:.2}A{r0} {r0} 0 0 0 {x3:.2} {y3:.2}Z"
    )
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;")
}

/// Both wheels. `drag` is an extra rotation of the inner wheel in degrees,
/// used while the user is turning it.
pub fn wheel(st: &State, drag: f64) -> String {
    let mut o = String::with_capacity(12_000);
    o.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-200 -200 400 400" class="wheel" role="img" aria-label="Chord wheel">"#);

    let mode_tonic = st.scale()[st.degree].pc();
    let chord: Vec<u8> = st.chord_notes(st.degree).iter().map(|n| n.pc()).collect();
    let formula = mode_formula(st.degree);

    // Outer wheel: fixed note names.
    o.push_str(r#"<g class="outer">"#);
    for pc in 0..12u8 {
        let a = st.pos(pc) as f64 * 30.0;
        let deg = st.degree_of(pc);
        let mut cls = String::new();
        if deg.is_some() {
            cls.push_str(" in");
        }
        if chord.contains(&pc) {
            cls.push_str(" ct");
        }
        if pc == mode_tonic {
            cls.push_str(" tonic");
        }
        let _ = write!(
            o,
            r#"<path class="o{cls}" data-pc="{pc}" d="{}"/>"#,
            wedge(R_SPLIT, R_OUTER, a - 15.0, a + 15.0)
        );
        let (x, y) = polar(177.0, a);
        let _ = write!(
            o,
            r#"<text class="oname{cls}" x="{x:.1}" y="{y:.1}">{}</text>"#,
            esc(&st.outer_name(pc))
        );
        if let Some(d) = deg {
            // Step of this note within the selected mode, e.g. ♭3 in Dorian.
            let step = (d + 7 - st.degree) % 7;
            let (x, y) = polar(157.0, a);
            let _ = write!(o, r#"<text class="ostep{cls}" x="{x:.1}" y="{y:.1}">{}</text>"#, formula[step]);
        }
    }
    o.push_str("</g>");

    // Inner wheel: degrees, turned so that I sits under the key.
    let base = st.pos(st.key) as f64 * 30.0 + drag;
    o.push_str(r#"<g class="inner" data-inner="1">"#);
    let _ = write!(o, r#"<circle class="disc" r="{R_INNER}"/>"#);
    for k in 0..12u8 {
        let a = base + st.pos(k) as f64 * 30.0;
        match MAJOR.iter().position(|&m| m == k) {
            Some(d) => {
                let mut cls = format!("i f{}", FUNCTIONS[d].to_lowercase());
                if d == st.degree {
                    cls.push_str(" sel");
                }
                let _ = write!(
                    o,
                    r#"<path class="{cls}" data-deg="{d}" d="{}"/>"#,
                    wedge(R_HUB + 3.0, R_INNER - 3.0, a - 14.0, a + 14.0)
                );
                // Labels stack vertically around the middle of the wedge so they
                // stay upright and readable at any rotation.
                let roman = st.roman(d);
                let (x, y) = polar(104.0, a);
                let long = if roman.chars().count() > 5 { " long" } else { "" };
                let _ = write!(o, r#"<text class="roman{long}" x="{x:.1}" y="{:.1}">{}</text>"#, y - 12.0, esc(&roman));
                let _ = write!(o, r#"<text class="mode" x="{x:.1}" y="{:.1}">{}</text>"#, y + 4.0, MODES[d]);
                let _ = write!(o, r#"<text class="iname" x="{x:.1}" y="{:.1}">{}</text>"#, y + 16.0, esc(&st.chord_name(d)));
            }
            None => {
                let (x0, y0) = polar(R_INNER - 8.0, a);
                let (x1, y1) = polar(R_INNER - 22.0, a);
                let _ = write!(o, r#"<line class="tick" x1="{x0:.1}" y1="{y0:.1}" x2="{x1:.1}" y2="{y1:.1}"/>"#);
            }
        }
    }
    // Pointer marking degree I, like the arrow drawn on a paper wheel.
    let (px, py) = polar(R_INNER + 1.0, base);
    let (lx, ly) = polar(R_INNER - 9.0, base - 4.0);
    let (rx, ry) = polar(R_INNER - 9.0, base + 4.0);
    let _ = write!(o, r#"<path class="pointer" d="M{px:.1} {py:.1}L{lx:.1} {ly:.1}L{rx:.1} {ry:.1}Z"/>"#);
    o.push_str("</g>");

    // Hub: what is selected right now.
    let _ = write!(o, r#"<circle class="hub" r="{R_HUB}"/>"#);
    let _ = write!(o, r#"<text class="hubchord" x="0" y="-8">{}</text>"#, esc(&st.chord_name(st.degree)));
    let _ = write!(o, r#"<text class="hubmode" x="0" y="14">{}</text>"#, esc(&st.mode_name()));
    let _ = write!(o, r#"<text class="hubkey" x="0" y="30">key of {} major</text>"#, esc(&st.root().name()));
    o.push_str("</svg>");
    o
}

// Staff geometry: staff step 30 is E4 (bottom line); each step is half a space.
const SPACE: f64 = 12.0;
const BOTTOM: f64 = 98.0;
const GLYPH: f64 = SPACE / 250.0;

fn step_y(step: i32) -> f64 {
    BOTTOM - (step - 30) as f64 * SPACE / 2.0
}

fn staff_step(n: Spelled, midi: u8) -> i32 {
    octave_of(n, midi) * 7 + n.letter as i32
}

fn ledgers(o: &mut String, x: f64, step: i32) {
    let mut s = 28;
    while s >= step {
        let y = step_y(s);
        let _ = write!(o, r#"<line class="ledger" x1="{:.1}" x2="{:.1}" y1="{y}" y2="{y}"/>"#, x - 11.0, x + 11.0);
        s -= 2;
    }
    let mut s = 40;
    while s <= step {
        let y = step_y(s);
        let _ = write!(o, r#"<line class="ledger" x1="{:.1}" x2="{:.1}" y1="{y}" y2="{y}"/>"#, x - 11.0, x + 11.0);
        s += 2;
    }
}

fn notehead(o: &mut String, x: f64, step: i32, cls: &str, midi: u8) {
    ledgers(o, x, step);
    let y = step_y(step);
    let _ = write!(
        o,
        r#"<g class="note {cls}" data-midi="{midi}"><rect class="hit" x="{:.1}" y="{:.1}" width="30" height="40"/><ellipse cx="{x:.1}" cy="{y:.1}" rx="7.2" ry="5.1" transform="rotate(-20 {x:.1} {y:.1})"/></g>"#,
        x - 15.0,
        y - 20.0
    );
}

/// Treble staff with the key signature, the selected mode as a scale, and the
/// selected chord stacked at the end.
pub fn staff(st: &State) -> String {
    let sig = st.signature();
    let mode = st.mode_notes();
    let mode_midi = st.mode_midi();
    let formula = mode_formula(st.degree);
    let chord_pcs: Vec<u8> = st.chord_notes(st.degree).iter().map(|n| n.pc()).collect();

    let note_gap = 44.0;
    let sig_w = sig.unsigned_abs() as f64 * 11.0;
    let x_notes = 70.0 + sig_w + 18.0;
    let x_bar = x_notes + 8.0 * note_gap - 10.0;
    let x_chord = x_bar + 46.0;
    let width = x_chord + 46.0;

    let mut o = String::with_capacity(8_000);
    let _ = write!(
        o,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width:.0} 172" class="staff" role="img" aria-label="Staff showing {}">"#,
        esc(&st.mode_name())
    );
    for line in 0..5 {
        let y = BOTTOM - line as f64 * SPACE;
        let _ = write!(o, r#"<line class="sline" x1="8" x2="{x_bar:.1}" y1="{y}" y2="{y}"/>"#);
        let _ = write!(o, r#"<line class="sline" x1="{x_bar:.1}" x2="{:.1}" y1="{y}" y2="{y}"/>"#, width - 8.0);
    }
    for x in [8.0, x_bar, width - 8.0] {
        let _ = write!(o, r#"<line class="bar" x1="{x:.1}" x2="{x:.1}" y1="{}" y2="{BOTTOM}"/>"#, BOTTOM - 4.0 * SPACE);
    }
    let _ = write!(
        o,
        r#"<path class="glyph" d="{}" transform="translate(16 {}) scale({GLYPH} -{GLYPH})"/>"#,
        glyphs::TREBLE_CLEF,
        step_y(30)
    );

    // Key signature, in the standard order and positions for treble clef.
    let (glyph, steps, dy): (&str, [i32; 7], f64) = if sig >= 0 {
        (glyphs::SHARP, [38, 35, 39, 36, 33, 37, 34], 131.0 * GLYPH)
    } else {
        (glyphs::FLAT, [34, 37, 33, 36, 32, 35, 31], 100.0 * GLYPH)
    };
    for (i, s) in steps.iter().take(sig.unsigned_abs() as usize).enumerate() {
        let x = 66.0 + i as f64 * 11.0;
        let _ = write!(
            o,
            r#"<path class="glyph" d="{glyph}" transform="translate({x:.1} {:.1}) scale({GLYPH} -{GLYPH})"/>"#,
            step_y(*s) + dy
        );
    }

    // The mode as a scale, tonic to tonic.
    for i in 0..8 {
        let n = mode[i % 7];
        let x = x_notes + i as f64 * note_gap;
        let step = staff_step(n, mode_midi[i]);
        let mut cls = String::new();
        if chord_pcs.contains(&n.pc()) {
            cls.push_str("ct");
        }
        if i % 7 == 0 {
            cls.push_str(" tonic");
        }
        notehead(&mut o, x, step, &cls, mode_midi[i]);
        let _ = write!(o, r#"<text class="lname {cls}" x="{x:.1}" y="138">{}</text>"#, esc(&n.name()));
        let _ = write!(o, r#"<text class="lstep" x="{x:.1}" y="156">{}</text>"#, formula[i % 7]);
    }

    // The chord, rooted at the octave of middle C so it sits on the staff.
    let notes = st.chord_notes(st.degree);
    let pcs: Vec<u8> = notes.iter().map(|n| n.pc()).collect();
    let midi = ascending_midi(&pcs, 60);
    let _ = write!(o, r#"<g class="chord" data-chord="1">"#);
    for (n, m) in notes.iter().zip(&midi) {
        notehead(&mut o, x_chord, staff_step(*n, *m), "ct", *m);
    }
    let _ = write!(o, r#"<text class="lname ct" x="{x_chord:.1}" y="138">{}</text>"#, esc(&st.chord_name(st.degree)));
    let _ = write!(o, r#"<text class="lstep" x="{x_chord:.1}" y="156">{}</text>"#, esc(&st.roman(st.degree)));
    o.push_str("</g></svg>");
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_every_key_and_degree() {
        for key in 0..12 {
            for degree in 0..7 {
                let st = State { key, degree, ..State::default() };
                assert!(wheel(&st, 0.0).ends_with("</svg>"));
                let s = staff(&st);
                assert!(s.contains("data-midi"));
            }
        }
    }

    #[test]
    fn scale_stays_near_the_staff() {
        for key in 0..12 {
            for degree in 0..7 {
                let st = State { key, degree, ..State::default() };
                for (n, m) in st.mode_notes().iter().zip(st.mode_midi()) {
                    let s = staff_step(*n, m);
                    assert!((27..=42).contains(&s), "{} at step {s}", n.name());
                }
            }
        }
    }
}
