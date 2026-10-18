//! Music theory for the wheel: pitch classes, note spelling, the major scale,
//! its seven modes and the diatonic chords built on each degree.

/// Semitone offsets of the major scale degrees from the tonic.
pub const MAJOR: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];

/// Pitch class of each natural letter, C D E F G A B.
const LETTER_PC: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
const LETTERS: [char; 7] = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];

pub const ROMAN: [&str; 7] = ["I", "ii", "iii", "IV", "V", "vi", "vii"];

pub const MODES: [&str; 7] = [
    "Ionian",
    "Dorian",
    "Phrygian",
    "Lydian",
    "Mixolydian",
    "Aeolian",
    "Locrian",
];

pub const MODE_DESCRIPTIONS: [&str; 7] = [
    "The major scale. Bright, happy and settled: most pop and folk songs live here.",
    "A minor scale with a raised 6th. Soulful and jazzy, common in funk, jazz and Celtic music.",
    "A minor scale with a lowered 2nd. Dark and exotic, with a Spanish or flamenco flavour.",
    "A major scale with a raised 4th. Dreamy and floating, loved by film composers.",
    "A major scale with a lowered 7th. Bluesy and earthy, heard in rock, blues and folk.",
    "The natural minor scale. Sad, serious or epic: the home of most minor-key songs.",
    "A diminished, unstable sound with a lowered 2nd and 5th. Rarely used as a home key.",
];

/// What a chord tends to do inside a key, in beginner terms.
pub const FUNCTIONS: [&str; 7] = [
    "Home",
    "Away",
    "Home",
    "Away",
    "Tension",
    "Home",
    "Tension",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quality {
    Maj7,
    Min7,
    Dom7,
    HalfDim7,
    Major,
    Minor,
    Diminished,
}

impl Quality {
    /// Quality of the chord on major scale degree `d` (0 = I).
    pub fn diatonic(d: usize, sevenths: bool) -> Quality {
        use Quality::*;
        if sevenths {
            [Maj7, Min7, Min7, Maj7, Dom7, Min7, HalfDim7][d]
        } else {
            [Major, Minor, Minor, Major, Major, Minor, Diminished][d]
        }
    }

    /// Suffix written after the root, e.g. "m7" in "Dm7".
    pub fn symbol(self) -> &'static str {
        match self {
            Quality::Maj7 => "maj7",
            Quality::Min7 => "m7",
            Quality::Dom7 => "7",
            Quality::HalfDim7 => "m7♭5",
            Quality::Major => "",
            Quality::Minor => "m",
            Quality::Diminished => "°",
        }
    }

    /// Suffix written after the roman numeral on the wheel, e.g. "m7" in "iim7".
    pub fn roman_suffix(self) -> &'static str {
        match self {
            Quality::Major | Quality::Minor => "",
            q => q.symbol(),
        }
    }

    pub fn long_name(self) -> &'static str {
        match self {
            Quality::Maj7 => "major seventh",
            Quality::Min7 => "minor seventh",
            Quality::Dom7 => "dominant seventh",
            Quality::HalfDim7 => "half-diminished seventh",
            Quality::Major => "major triad",
            Quality::Minor => "minor triad",
            Quality::Diminished => "diminished triad",
        }
    }
}

/// How note names are written on the outer wheel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accidentals {
    Sharps,
    Flats,
    /// Follow the spelling of the current key.
    Auto,
}

impl Accidentals {
    pub fn from_index(i: u8) -> Accidentals {
        match i {
            0 => Accidentals::Sharps,
            1 => Accidentals::Flats,
            _ => Accidentals::Auto,
        }
    }
}

/// A note name: a letter plus a number of sharps (positive) or flats (negative).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spelled {
    pub letter: u8,
    pub acc: i8,
}

impl Spelled {
    pub fn pc(self) -> u8 {
        (LETTER_PC[self.letter as usize] as i32 + self.acc as i32).rem_euclid(12) as u8
    }

    pub fn name(self) -> String {
        let mut s = String::new();
        s.push(LETTERS[self.letter as usize]);
        let sign = if self.acc > 0 { "♯" } else { "♭" };
        for _ in 0..self.acc.unsigned_abs() {
            s.push_str(sign);
        }
        s
    }

    /// Spell pitch class `pc` using `letter`, choosing the nearest accidental.
    fn with_letter(letter: u8, pc: u8) -> Spelled {
        let diff = (pc as i32 - LETTER_PC[letter as usize] as i32).rem_euclid(12);
        let acc = if diff > 6 { diff - 12 } else { diff };
        Spelled { letter, acc: acc as i8 }
    }

    fn from_pc(pc: u8, flats: bool) -> Spelled {
        let pc = pc % 12;
        if let Some(l) = LETTER_PC.iter().position(|&p| p == pc) {
            return Spelled { letter: l as u8, acc: 0 };
        }
        if flats {
            Spelled::with_letter(LETTER_PC.iter().position(|&p| p == pc + 1).unwrap() as u8, pc)
        } else {
            Spelled::with_letter(LETTER_PC.iter().position(|&p| p == pc - 1).unwrap() as u8, pc)
        }
    }
}

/// Name of a pitch class using plain sharps or flats.
pub fn chromatic_name(pc: u8, flats: bool) -> String {
    Spelled::from_pc(pc, flats).name()
}

/// The conventional spelling of the major key whose tonic is `pc`.
/// Keys that are written either way (C♯/D♭ and F♯/G♭) follow the preference.
pub fn key_root(pc: u8, acc: Accidentals) -> Spelled {
    let pc = pc % 12;
    let prefer_flats = match acc {
        Accidentals::Sharps => false,
        Accidentals::Flats => true,
        Accidentals::Auto => pc == 1,
    };
    let flats = match pc {
        1 | 6 => prefer_flats,
        3 | 8 | 10 => true,
        _ => false,
    };
    Spelled::from_pc(pc, flats)
}

/// The seven notes of the major scale starting on `root`, one per letter.
pub fn major_scale(root: Spelled) -> [Spelled; 7] {
    let mut out = [root; 7];
    for (i, n) in out.iter_mut().enumerate() {
        let letter = (root.letter + i as u8) % 7;
        *n = Spelled::with_letter(letter, (root.pc() + MAJOR[i]) % 12);
    }
    out
}

/// Number of sharps (positive) or flats (negative) in the key signature.
pub fn key_signature(scale: &[Spelled; 7]) -> i8 {
    scale.iter().map(|n| n.acc).sum()
}

pub fn key_signature_text(sig: i8) -> String {
    match sig {
        0 => "no sharps or flats".to_string(),
        1 => "1 sharp".to_string(),
        -1 => "1 flat".to_string(),
        s if s > 0 => format!("{s} sharps"),
        s => format!("{} flats", -s),
    }
}

/// Scale-step formula of the mode on degree `d`, e.g. Dorian = 1 2 ♭3 4 5 6 ♭7.
pub fn mode_formula(d: usize) -> [String; 7] {
    let mut out: [String; 7] = Default::default();
    for (i, s) in out.iter_mut().enumerate() {
        let interval = (MAJOR[(d + i) % 7] as i32 - MAJOR[d] as i32).rem_euclid(12);
        let diff = interval - MAJOR[i] as i32;
        let acc = match diff {
            -1 => "♭",
            1 => "♯",
            _ => "",
        };
        *s = format!("{acc}{}", i + 1);
    }
    out
}

/// Indexes into the key's scale of the notes of the chord on degree `d`.
pub fn chord_degrees(d: usize, sevenths: bool) -> Vec<usize> {
    let n = if sevenths { 4 } else { 3 };
    (0..n).map(|i| (d + 2 * i) % 7).collect()
}

/// MIDI notes for a pitch-class sequence, each one above the previous.
pub fn ascending_midi(pcs: &[u8], start: u8) -> Vec<u8> {
    let mut out = Vec::with_capacity(pcs.len());
    let mut prev: Option<u8> = None;
    for &pc in pcs {
        let mut m = match prev {
            None => start + pc,
            Some(p) => p - (p % 12) + pc,
        };
        if let Some(p) = prev {
            while m <= p {
                m += 12;
            }
        }
        out.push(m);
        prev = Some(m);
    }
    out
}

/// Octave number (scientific pitch notation) of a spelled note sounding at `midi`.
pub fn octave_of(n: Spelled, midi: u8) -> i32 {
    let natural = midi as i32 - n.acc as i32;
    natural.div_euclid(12) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(s: &[Spelled]) -> Vec<String> {
        s.iter().map(|n| n.name()).collect()
    }

    #[test]
    fn c_major() {
        let s = major_scale(key_root(0, Accidentals::Sharps));
        assert_eq!(names(&s), ["C", "D", "E", "F", "G", "A", "B"]);
        assert_eq!(key_signature(&s), 0);
    }

    #[test]
    fn flat_and_sharp_keys() {
        let f = major_scale(key_root(5, Accidentals::Sharps));
        assert_eq!(names(&f), ["F", "G", "A", "B♭", "C", "D", "E"]);
        assert_eq!(key_signature(&f), -1);
        let fs = major_scale(key_root(6, Accidentals::Sharps));
        assert_eq!(names(&fs), ["F♯", "G♯", "A♯", "B", "C♯", "D♯", "E♯"]);
        assert_eq!(key_signature(&fs), 6);
        let gb = major_scale(key_root(6, Accidentals::Flats));
        assert_eq!(key_signature(&gb), -6);
        // E♭ even when sharps are preferred: D♯ major would need double sharps.
        assert_eq!(key_root(3, Accidentals::Sharps).name(), "E♭");
    }

    #[test]
    fn every_key_has_one_kind_of_accidental() {
        for pc in 0..12 {
            for a in [Accidentals::Sharps, Accidentals::Flats, Accidentals::Auto] {
                let s = major_scale(key_root(pc, a));
                let sharps = s.iter().any(|n| n.acc > 0);
                let flats = s.iter().any(|n| n.acc < 0);
                assert!(!(sharps && flats), "key {pc}");
                assert!(s.iter().all(|n| n.acc.abs() <= 1), "key {pc}");
                assert!(key_signature(&s).abs() <= 7);
            }
        }
    }

    #[test]
    fn formulas() {
        assert_eq!(mode_formula(1).join(" "), "1 2 ♭3 4 5 6 ♭7");
        assert_eq!(mode_formula(3).join(" "), "1 2 3 ♯4 5 6 7");
        assert_eq!(mode_formula(6).join(" "), "1 ♭2 ♭3 4 ♭5 ♭6 ♭7");
    }

    #[test]
    fn chords() {
        assert_eq!(chord_degrees(1, true), [1, 3, 5, 0]);
        assert_eq!(Quality::diatonic(4, true).symbol(), "7");
        assert_eq!(ascending_midi(&[2, 5, 9, 0], 48), [50, 53, 57, 60]);
    }

    #[test]
    fn octaves() {
        let bs = Spelled { letter: 6, acc: 1 };
        assert_eq!(octave_of(bs, 60), 3); // B♯3 sounds like C4
        assert_eq!(octave_of(Spelled { letter: 0, acc: 0 }, 60), 4);
    }
}
