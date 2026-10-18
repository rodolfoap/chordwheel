//! The wheel's position and options, and everything derived from them.

use crate::theory::*;

#[derive(Clone, Debug)]
pub struct State {
    /// Pitch class under degree I (the parent major key).
    pub key: u8,
    /// Selected degree, 0 = I. It picks both the highlighted chord and the mode.
    pub degree: usize,
    pub acc: Accidentals,
    pub sevenths: bool,
    /// Lay both wheels out around the circle of fifths instead of chromatically.
    pub fifths: bool,
}

impl Default for State {
    fn default() -> Self {
        State { key: 0, degree: 0, acc: Accidentals::Sharps, sevenths: true, fifths: false }
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

    pub fn root(&self) -> Spelled {
        key_root(self.key, self.acc)
    }

    pub fn scale(&self) -> [Spelled; 7] {
        major_scale(self.root())
    }

    pub fn signature(&self) -> i8 {
        key_signature(&self.scale())
    }

    /// Degree of pitch class `pc` in the current key, if it belongs to it.
    pub fn degree_of(&self, pc: u8) -> Option<usize> {
        let off = (pc as i32 - self.key as i32).rem_euclid(12) as u8;
        MAJOR.iter().position(|&m| m == off)
    }

    /// Name shown on the outer wheel.
    pub fn outer_name(&self, pc: u8) -> String {
        match self.acc {
            Accidentals::Sharps => chromatic_name(pc, false),
            Accidentals::Flats => chromatic_name(pc, true),
            Accidentals::Auto => match self.degree_of(pc) {
                Some(d) => self.scale()[d].name(),
                None => chromatic_name(pc, self.signature() < 0),
            },
        }
    }

    pub fn quality(&self, d: usize) -> Quality {
        Quality::diatonic(d, self.sevenths)
    }

    pub fn roman(&self, d: usize) -> String {
        format!("{}{}", ROMAN[d], self.quality(d).roman_suffix())
    }

    pub fn chord_notes(&self, d: usize) -> Vec<Spelled> {
        let s = self.scale();
        chord_degrees(d, self.sevenths).into_iter().map(|i| s[i]).collect()
    }

    pub fn chord_name(&self, d: usize) -> String {
        format!("{}{}", self.scale()[d].name(), self.quality(d).symbol())
    }

    /// The selected mode's notes, starting on its own tonic.
    pub fn mode_notes(&self) -> Vec<Spelled> {
        let s = self.scale();
        (0..7).map(|i| s[(self.degree + i) % 7]).collect()
    }

    pub fn mode_name(&self) -> String {
        format!("{} {}", self.scale()[self.degree].name(), MODES[self.degree])
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
        self.key = (tonic as i32 - MAJOR[d] as i32).rem_euclid(12) as u8;
        self.degree = d;
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
    }
}
