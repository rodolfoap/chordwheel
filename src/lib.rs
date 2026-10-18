//! ChordWheel: a digital version of the paper wheel for finding the notes and
//! chords of a key and its modes. The page's JavaScript only handles input and
//! sound; the music theory and the drawing live here.

mod glyphs;
pub mod render;
pub mod state;
pub mod theory;

use serde::Serialize;
use wasm_bindgen::prelude::*;

use state::State;
use theory::*;

/// Well-known progressions, as degrees of the major key (0 = I).
const PROGRESSIONS: [(&str, &str, &[usize]); 7] = [
    ("Pop", "The four chords behind countless hits.", &[0, 4, 5, 3]),
    ("Doo-wop", "The 1950s ballad sound.", &[0, 5, 3, 4]),
    ("Classic", "Folk, country and early rock and roll.", &[0, 3, 4, 0]),
    ("Jazz ii–V–I", "The most common move in jazz.", &[1, 4, 0]),
    ("Sad pop", "The pop progression starting on the minor chord.", &[5, 3, 0, 4]),
    ("Andalusian", "A descending minor cadence with a flamenco feel.", &[5, 4, 3, 2]),
    ("Canon", "Pachelbel's Canon and its many descendants.", &[0, 4, 5, 2, 3, 0, 3, 4]),
];

#[derive(Serialize)]
struct ChordInfo {
    degree: usize,
    roman: String,
    name: String,
    quality: &'static str,
    notes: Vec<String>,
    function: &'static str,
    mode: &'static str,
    midi: Vec<u8>,
}

#[derive(Serialize)]
struct ProgressionInfo {
    name: &'static str,
    description: &'static str,
    roman: String,
    chords: String,
    degrees: Vec<usize>,
}

#[derive(Serialize)]
struct Info {
    key: String,
    signature: String,
    relative_minor: String,
    mode: String,
    mode_name: &'static str,
    mode_description: &'static str,
    formula: Vec<String>,
    scale: Vec<String>,
    scale_midi: Vec<u8>,
    chord: usize,
    chords: Vec<ChordInfo>,
    progressions: Vec<ProgressionInfo>,
}

#[wasm_bindgen]
pub struct Wheel {
    st: State,
}

impl Default for Wheel {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Wheel {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Wheel {
        Wheel { st: State::default() }
    }

    #[wasm_bindgen(getter)]
    pub fn key(&self) -> u8 {
        self.st.key
    }
    #[wasm_bindgen(setter)]
    pub fn set_key(&mut self, pc: u8) {
        self.st.key = pc % 12;
    }

    #[wasm_bindgen(getter)]
    pub fn degree(&self) -> usize {
        self.st.degree
    }
    #[wasm_bindgen(setter)]
    pub fn set_degree(&mut self, d: usize) {
        self.st.degree = d % 7;
    }

    /// 0 = sharps, 1 = flats, 2 = follow the key.
    #[wasm_bindgen(getter)]
    pub fn accidentals(&self) -> u8 {
        self.st.acc as u8
    }
    #[wasm_bindgen(setter)]
    pub fn set_accidentals(&mut self, i: u8) {
        self.st.acc = Accidentals::from_index(i);
    }

    #[wasm_bindgen(getter)]
    pub fn sevenths(&self) -> bool {
        self.st.sevenths
    }
    #[wasm_bindgen(setter)]
    pub fn set_sevenths(&mut self, on: bool) {
        self.st.sevenths = on;
    }

    #[wasm_bindgen(getter)]
    pub fn fifths(&self) -> bool {
        self.st.fifths
    }
    #[wasm_bindgen(setter)]
    pub fn set_fifths(&mut self, on: bool) {
        self.st.fifths = on;
    }

    /// Turn the inner wheel by whole slots (positive is clockwise).
    pub fn rotate(&mut self, steps: i32) {
        self.st.rotate(steps);
    }

    /// Pick a mode while keeping its tonic, e.g. D Dorian to D Phrygian.
    pub fn set_mode_keep_tonic(&mut self, d: usize) {
        self.st.set_mode_keep_tonic(d % 7);
    }

    /// Pitch class of the selected mode's tonic.
    pub fn tonic(&self) -> u8 {
        self.st.scale()[self.st.degree].pc()
    }

    /// The wheels as SVG; `drag` turns the inner wheel by extra degrees.
    pub fn wheel_svg(&self, drag: f64) -> String {
        render::wheel(&self.st, drag)
    }

    pub fn staff_svg(&self) -> String {
        render::staff(&self.st)
    }

    pub fn chord_midi(&self, d: usize) -> Vec<u8> {
        self.st.chord_midi(d % 7)
    }

    pub fn name_of(&self, pc: u8) -> String {
        self.st.outer_name(pc % 12)
    }

    /// Everything the info panel shows, as JSON.
    pub fn info_json(&self) -> String {
        let st = &self.st;
        let scale = st.scale();
        let chords = (0..7)
            .map(|d| ChordInfo {
                degree: d,
                roman: st.roman(d),
                name: st.chord_name(d),
                quality: st.quality(d).long_name(),
                notes: st.chord_notes(d).iter().map(|n| n.name()).collect(),
                function: FUNCTIONS[d],
                mode: MODES[d],
                midi: st.chord_midi(d),
            })
            .collect();
        let progressions = PROGRESSIONS
            .iter()
            .map(|(name, description, degs)| ProgressionInfo {
                name,
                description,
                roman: degs.iter().map(|&d| st.roman(d)).collect::<Vec<_>>().join(" – "),
                chords: degs.iter().map(|&d| st.chord_name(d)).collect::<Vec<_>>().join(" – "),
                degrees: degs.to_vec(),
            })
            .collect();
        let info = Info {
            key: format!("{} major", scale[0].name()),
            signature: key_signature_text(st.signature()),
            relative_minor: format!("{} minor", scale[5].name()),
            mode: st.mode_name(),
            mode_name: MODES[st.degree],
            mode_description: MODE_DESCRIPTIONS[st.degree],
            formula: mode_formula(st.degree).to_vec(),
            scale: st.mode_notes().iter().map(|n| n.name()).collect(),
            scale_midi: st.mode_midi(),
            chord: st.degree,
            chords,
            progressions,
        };
        serde_json::to_string(&info).unwrap_or_default()
    }
}

/// Radii of the wheel drawing, so the page can tell the rings apart.
#[wasm_bindgen]
pub fn ring_radii() -> Vec<f64> {
    vec![render::R_HUB, render::R_INNER, render::R_SPLIT, render::R_OUTER]
}
