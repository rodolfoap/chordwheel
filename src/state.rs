//! The wheel's position and options, and everything derived from them.

use crate::theory::*;

#[derive(Clone, Debug)]
pub struct State {
    /// Pitch class under degree I (the tonic of the wheel's scale).
    pub key: u8,
    /// Selected degree, 0 = I. It picks both the highlighted chord and the mode.
    pub degree: usize,
    pub acc: Accidentals,
    pub sevenths: bool,
    /// Lay both wheels out around the circle of fifths instead of chromatically.
    pub fifths: bool,
    /// Major (with its natural minor as vi) or melodic minor.
    pub family: Family,
}

impl Default for State {
    fn default() -> Self {
        State {
            key: 0,
            degree: 0,
            acc: Accidentals::Sharps,
            sevenths: true,
            fifths: false,
            family: Family::Major,
        }
    }
}

impl State {
    /// Slot (0..12, clockwise from the top) where pitch class `pc` sits.
    /// Multiplying by 7 is its own inverse mod 12, so this also maps slots back.
    pub fn pos(&self, pc: u8) -> u8 {
        if self.fifths {
            (pc as u32 * 7 % 12) as u8
        } else {
            pc % 12
        }
    }

    /// Turn the inner wheel by `steps` slots clockwise.
    pub fn rotate(&mut self, steps: i32) {
        let p = (self.pos(self.key) as i32 + steps).rem_euclid(12) as u8;
        self.key = self.pos(p);
    }

    pub fn intervals(&self) -> [u8; 7] {
        self.family.intervals()
    }

    pub fn root(&self) -> Spelled {
        tonic_spelling(self.key, self.acc, self.family)
    }

    pub fn scale(&self) -> [Spelled; 7] {
        scale_from(self.root(), self.intervals())
    }

    /// E.g. "C major" or "C melodic minor".
    pub fn key_name(&self) -> String {
        format!("{} {}", self.root().name(), self.family.name())
    }

    fn signature_scale(&self) -> [Spelled; 7] {
        signature_scale(self.root(), self.family)
    }

    pub fn signature(&self) -> i8 {
        key_signature(&self.signature_scale())
    }

    /// Accidental the key signature gives to `letter`.
    pub fn signature_acc(&self, letter: u8) -> i8 {
        self.signature_scale().iter().find(|n| n.letter == letter).map_or(0, |n| n.acc)
    }

    /// Degree of pitch class `pc` in the current key, if it belongs to it.
    pub fn degree_of(&self, pc: u8) -> Option<usize> {
        let off = (pc as i32 - self.key as i32).rem_euclid(12) as u8;
        self.intervals().iter().position(|&m| m == off)
    }

    /// Name of `pc` as the current key spells it, or plainly if it is outside the key.
    pub fn key_spelling(&self, pc: u8) -> String {
        match self.degree_of(pc) {
            Some(d) => self.scale()[d].name(),
            None => chromatic_name(pc, self.signature() < 0),
        }
    }

    /// Name shown on the outer wheel.
    pub fn outer_name(&self, pc: u8) -> String {
        match self.acc {
            Accidentals::Sharps => chromatic_name(pc, false),
            Accidentals::Flats => chromatic_name(pc, true),
            Accidentals::Auto => self.key_spelling(pc),
        }
    }

    pub fn quality(&self, d: usize) -> Quality {
        let pcs: Vec<u8> = self.chord_notes(d).iter().map(|n| n.pc()).collect();
        let offsets: Vec<u8> = pcs[1..].iter().map(|&p| (p + 12 - pcs[0]) % 12).collect();
        Quality::from_offsets(&offsets)
    }

    pub fn roman(&self, d: usize) -> String {
        roman(d, self.intervals(), self.quality(d))
    }

    pub fn chord_notes(&self, d: usize) -> Vec<Spelled> {
        let s = self.scale();
        chord_degrees(d, self.sevenths).into_iter().map(|i| s[i]).collect()
    }

    pub fn chord_name(&self, d: usize) -> String {
        format!("{}{}", self.scale()[d].name(), self.quality(d).symbol())
    }

    pub fn formula(&self) -> [String; 7] {
        mode_formula(self.degree, self.intervals())
    }

    /// The selected mode's notes, starting on its own tonic.
    pub fn mode_notes(&self) -> Vec<Spelled> {
        let s = self.scale();
        (0..7).map(|i| s[(self.degree + i) % 7]).collect()
    }

    pub fn mode_name(&self) -> String {
        format!("{} {}", self.scale()[self.degree].name(), self.family.modes()[self.degree])
    }

    /// MIDI notes of the chord on degree `d`, rooted in the octave below middle C.
    pub fn chord_midi(&self, d: usize) -> Vec<u8> {
        let pcs: Vec<u8> = self.chord_notes(d).iter().map(|n| n.pc()).collect();
        ascending_midi(&pcs, 48)
    }

    /// MIDI notes of the selected mode from its tonic up to the octave.
    pub fn mode_midi(&self) -> Vec<u8> {
        let mut pcs: Vec<u8> = self.mode_notes().iter().map(|n| n.pc()).collect();
        pcs.push(pcs[0]);
        ascending_midi(&pcs, 60)
    }

    /// Change the mode while keeping its tonic note, by turning the wheel.
    /// E.g. from D Dorian, choosing Phrygian gives D Phrygian.
    pub fn set_mode_keep_tonic(&mut self, d: usize) {
        let tonic = self.scale()[self.degree].pc();
        self.degree = d;
        self.set_tonic(tonic);
    }

    /// Switch scale family, keeping the selected degree and its tonic.
    pub fn set_family(&mut self, family: Family) {
        let tonic = self.scale()[self.degree].pc();
        self.family = family;
        self.set_tonic(tonic);
    }

    /// Turn the wheel so the selected mode starts on `pc`.
    pub fn set_tonic(&mut self, pc: u8) {
        self.key = (pc as i32 - self.intervals()[self.degree] as i32).rem_euclid(12) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_dorian_in_c() {
        let s = State { degree: 1, ..State::default() };
        assert_eq!(s.mode_name(), "D Dorian");
        assert_eq!(s.chord_name(1), "Dm7");
        assert_eq!(s.roman(6), "viim7♭5");
        assert_eq!(s.chord_midi(4), [55, 59, 62, 65]);
    }

    #[test]
    fn diatonic_qualities() {
        let s = State::default();
        let romans: Vec<String> = (0..7).map(|d| s.roman(d)).collect();
        assert_eq!(romans, ["Imaj7", "iim7", "iiim7", "IVmaj7", "V7", "vim7", "viim7♭5"]);
        let t = State { sevenths: false, ..State::default() };
        let romans: Vec<String> = (0..7).map(|d| t.roman(d)).collect();
        assert_eq!(romans, ["I", "ii", "iii", "IV", "V", "vi", "vii°"]);
    }

    #[test]
    fn melodic_minor_chords() {
        let s = State { family: Family::MelodicMinor, ..State::default() };
        let names: Vec<String> = (0..7).map(|d| s.chord_name(d)).collect();
        assert_eq!(names, ["CmMaj7", "Dm7", "E♭maj7♯5", "F7", "G7", "Am7♭5", "Bm7♭5"]);
        let romans: Vec<String> = (0..7).map(|d| s.roman(d)).collect();
        assert_eq!(romans, ["imMaj7", "iim7", "♭IIImaj7♯5", "IV7", "V7", "vim7♭5", "viim7♭5"]);
        assert_eq!(s.key_name(), "C melodic minor");
        assert_eq!(s.signature(), -3);
        assert_eq!(s.signature_acc(5), -1); // A♭ in the signature, A♮ in the scale
        let alt = State { degree: 6, ..s };
        assert_eq!(alt.mode_name(), "B Altered");
    }

    #[test]
    fn rotation() {
        let mut s = State::default();
        s.rotate(1);
        assert_eq!(s.key, 1);
        s.fifths = true;
        s.key = 0;
        s.rotate(1);
        assert_eq!(s.key, 7); // one slot clockwise on the circle of fifths is G
        s.rotate(-2);
        assert_eq!(s.key, 5);
    }

    #[test]
    fn keep_tonic() {
        let mut s = State { degree: 1, ..State::default() };
        s.set_mode_keep_tonic(2);
        assert_eq!(s.mode_name(), "D Phrygian");
        assert_eq!(s.root().name(), "B♭");
        s.set_family(Family::MelodicMinor);
        assert_eq!(s.mode_name(), "D Lydian augmented");
        s.set_mode_keep_tonic(0);
        assert_eq!(s.mode_name(), "D Melodic minor");
    }
}
