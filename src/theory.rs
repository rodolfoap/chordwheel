//! Music theory for the wheel: pitch classes, note spelling, the major and
//! melodic minor scales, their seven modes and the chords built on each degree.

/// Semitone offsets of the major scale degrees from the tonic.
pub const MAJOR: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
/// The (ascending, or "jazz") melodic minor: major with a lowered 3rd.
pub const MELODIC_MINOR: [u8; 7] = [0, 2, 3, 5, 7, 9, 11];

/// Pitch class of each natural letter, C D E F G A B.
const LETTER_PC: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
const LETTERS: [char; 7] = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];
const NUMERALS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];

/// The scale the inner wheel is built from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Major,
    MelodicMinor,
}

impl Family {
    pub fn from_index(i: u8) -> Family {
        if i == 1 {
            Family::MelodicMinor
        } else {
            Family::Major
        }
    }

    pub fn intervals(self) -> [u8; 7] {
        match self {
            Family::Major => MAJOR,
            Family::MelodicMinor => MELODIC_MINOR,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Family::Major => "major",
            Family::MelodicMinor => "melodic minor",
        }
    }

    pub fn modes(self) -> [&'static str; 7] {
        match self {
            Family::Major => ["Ionian", "Dorian", "Phrygian", "Lydian", "Mixolydian", "Aeolian", "Locrian"],
            Family::MelodicMinor => [
                "Melodic minor",
                "Dorian ♭2",
                "Lydian augmented",
                "Lydian dominant",
                "Mixolydian ♭6",
                "Locrian ♮2",
                "Altered",
            ],
        }
    }

    /// Mode names short enough to fit in a slot of the wheel.
    pub fn short_modes(self) -> [&'static str; 7] {
        match self {
            Family::Major => self.modes(),
            Family::MelodicMinor => [
                "Mel. minor",
                "Dorian ♭2",
                "Lydian aug.",
                "Lydian dom.",
                "Mixo ♭6",
                "Locrian ♮2",
                "Altered",
            ],
        }
    }

    pub fn descriptions(self) -> [&'static str; 7] {
        match self {
            Family::Major => [
                "The major scale. Bright, happy and settled: most pop and folk songs live here.",
                "A minor scale with a raised 6th. Soulful and jazzy, common in funk, jazz and Celtic music.",
                "A minor scale with a lowered 2nd. Dark and exotic, with a Spanish or flamenco flavour.",
                "A major scale with a raised 4th. Dreamy and floating, loved by film composers.",
                "A major scale with a lowered 7th. Bluesy and earthy, heard in rock, blues and folk.",
                "The natural minor scale. Sad, serious or epic: the home of most minor-key songs.",
                "A diminished, unstable sound with a lowered 2nd and 5th. Rarely used as a home key.",
            ],
            Family::MelodicMinor => [
                "Minor with a raised 6th and 7th. Smooth and sophisticated: the jazz minor scale, used over minor-major seventh chords.",
                "Phrygian with a raised 6th. Dark and exotic, used over sus♭9 chords.",
                "Lydian with a raised 5th. Bright and floating, used over maj7♯5 chords.",
                "Lydian with a lowered 7th, also called the overtone or Simpsons scale. Used over dominant 7♯11 chords.",
                "Major with a lowered 6th and 7th. Bittersweet, a dominant sound leaning towards minor.",
                "Locrian with a natural 2nd. The usual choice over half-diminished (m7♭5) chords.",
                "Every tension a dominant chord can have (♭9, ♯9, ♯11, ♭13). The jazz sound over V7 resolving to minor.",
            ],
        }
    }

    /// What a chord tends to do inside the key, in beginner terms.
    pub fn functions(self) -> [&'static str; 7] {
        match self {
            Family::Major => ["Home", "Away", "Home", "Away", "Tension", "Home", "Tension"],
            Family::MelodicMinor => ["Home", "Away", "Home", "Away", "Tension", "Away", "Tension"],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quality {
    Maj7,
    Min7,
    Dom7,
    HalfDim7,
    MinMaj7,
    AugMaj7,
    Major,
    Minor,
    Diminished,
    Augmented,
}

impl Quality {
    /// Quality of a chord from its semitone offsets above the root
    /// (third, fifth and optionally seventh).
    pub fn from_offsets(offsets: &[u8]) -> Quality {
        use Quality::*;
        match offsets {
            [4, 7, 11] => Maj7,
            [3, 7, 10] => Min7,
            [4, 7, 10] => Dom7,
            [3, 6, 10] => HalfDim7,
            [3, 7, 11] => MinMaj7,
            [4, 8, 11] => AugMaj7,
            [3, 7] => Minor,
            [3, 6] => Diminished,
            [4, 8] => Augmented,
            _ => Major,
        }
    }

    /// Suffix written after the root, e.g. "m7" in "Dm7".
    pub fn symbol(self) -> &'static str {
        match self {
            Quality::Maj7 => "maj7",
            Quality::Min7 => "m7",
            Quality::Dom7 => "7",
            Quality::HalfDim7 => "m7♭5",
            Quality::MinMaj7 => "mMaj7",
            Quality::AugMaj7 => "maj7♯5",
            Quality::Major => "",
            Quality::Minor => "m",
            Quality::Diminished => "°",
            Quality::Augmented => "+",
        }
    }

    /// Suffix written after the roman numeral on the wheel, e.g. "m7" in "iim7".
    pub fn roman_suffix(self) -> &'static str {
        match self {
            Quality::Major | Quality::Minor => "",
            q => q.symbol(),
        }
    }

    pub fn is_minor(self) -> bool {
        matches!(
            self,
            Quality::Min7 | Quality::HalfDim7 | Quality::MinMaj7 | Quality::Minor | Quality::Diminished
        )
    }

    pub fn long_name(self) -> &'static str {
        match self {
            Quality::Maj7 => "major seventh",
            Quality::Min7 => "minor seventh",
            Quality::Dom7 => "dominant seventh",
            Quality::HalfDim7 => "half-diminished seventh",
            Quality::MinMaj7 => "minor-major seventh",
            Quality::AugMaj7 => "augmented major seventh",
            Quality::Major => "major triad",
            Quality::Minor => "minor triad",
            Quality::Diminished => "diminished triad",
            Quality::Augmented => "augmented triad",
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
    pub fn with_letter(letter: u8, pc: u8) -> Spelled {
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

/// Spelling of the tonic of a `family` scale on pitch class `pc`. A melodic
/// minor tonic is spelled so that its scale needs the fewest accidentals
/// (A♭ rather than G♯, which would need F𝄪); ties follow the minor key.
pub fn tonic_spelling(pc: u8, acc: Accidentals, family: Family) -> Spelled {
    match family {
        Family::Major => key_root(pc, acc),
        Family::MelodicMinor => {
            let minor = major_scale(key_root((pc + 3) % 12, acc))[5];
            let cost = |t: Spelled| -> i32 {
                scale_from(t, MELODIC_MINOR)
                    .iter()
                    .map(|n| if n.acc.abs() > 1 { 10 } else { n.acc.abs() as i32 })
                    .sum()
            };
            [minor, Spelled::from_pc(pc, false), Spelled::from_pc(pc, true)]
                .into_iter()
                .min_by_key(|&t| cost(t))
                .unwrap_or(minor)
        }
    }
}

/// The seven notes of a scale starting on `root`, one per letter.
pub fn scale_from(root: Spelled, intervals: [u8; 7]) -> [Spelled; 7] {
    let mut out = [root; 7];
    for (i, n) in out.iter_mut().enumerate() {
        let letter = (root.letter + i as u8) % 7;
        *n = Spelled::with_letter(letter, (root.pc() + intervals[i]) % 12);
    }
    out
}

pub fn major_scale(root: Spelled) -> [Spelled; 7] {
    scale_from(root, MAJOR)
}

/// The major scale whose key signature a `family` scale on `tonic` is written
/// with: the scale itself for major, the relative major for melodic minor.
pub fn signature_scale(tonic: Spelled, family: Family) -> [Spelled; 7] {
    match family {
        Family::Major => major_scale(tonic),
        Family::MelodicMinor => {
            major_scale(Spelled::with_letter((tonic.letter + 2) % 7, (tonic.pc() + 3) % 12))
        }
    }
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

/// Accidental written in front of a step that is `diff` semitones away from
/// the major scale's step, e.g. "♭" for -1.
fn step_accidental(diff: i32) -> &'static str {
    match diff {
        -2 => "♭♭",
        -1 => "♭",
        1 => "♯",
        2 => "♯♯",
        _ => "",
    }
}

/// Scale-step formula of the mode on degree `d`, e.g. Dorian = 1 2 ♭3 4 5 6 ♭7.
pub fn mode_formula(d: usize, intervals: [u8; 7]) -> [String; 7] {
    let mut out: [String; 7] = Default::default();
    for (i, s) in out.iter_mut().enumerate() {
        let interval = (intervals[(d + i) % 7] as i32 - intervals[d] as i32).rem_euclid(12);
        *s = format!("{}{}", step_accidental(interval - MAJOR[i] as i32), i + 1);
    }
    out
}

/// Roman numeral for the chord on degree `d`: upper case for major-third
/// chords, with ♭/♯ when the degree differs from the major scale's.
pub fn roman(d: usize, intervals: [u8; 7], q: Quality) -> String {
    let acc = step_accidental(intervals[d] as i32 - MAJOR[d] as i32);
    let numeral = if q.is_minor() { NUMERALS[d].to_lowercase() } else { NUMERALS[d].to_string() };
    format!("{acc}{numeral}{}", q.roman_suffix())
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
    fn melodic_minor() {
        let t = tonic_spelling(0, Accidentals::Auto, Family::MelodicMinor);
        let s = scale_from(t, MELODIC_MINOR);
        assert_eq!(names(&s), ["C", "D", "E♭", "F", "G", "A", "B"]);
        assert_eq!(key_signature(&signature_scale(t, Family::MelodicMinor)), -3);
        let a = tonic_spelling(9, Accidentals::Auto, Family::MelodicMinor);
        assert_eq!(names(&scale_from(a, MELODIC_MINOR)), ["A", "B", "C", "D", "E", "F♯", "G♯"]);
        assert_eq!(tonic_spelling(10, Accidentals::Auto, Family::MelodicMinor).name(), "B♭");
        assert_eq!(tonic_spelling(8, Accidentals::Auto, Family::MelodicMinor).name(), "A♭");
        assert_eq!(tonic_spelling(3, Accidentals::Sharps, Family::MelodicMinor).name(), "E♭");
        assert_eq!(tonic_spelling(6, Accidentals::Flats, Family::MelodicMinor).name(), "F♯");
        for pc in 0..12 {
            for a in [Accidentals::Sharps, Accidentals::Flats, Accidentals::Auto] {
                let t = tonic_spelling(pc, a, Family::MelodicMinor);
                assert!(scale_from(t, MELODIC_MINOR).iter().all(|n| n.acc.abs() <= 1), "{pc}");
            }
        }
    }

    #[test]
    fn formulas() {
        assert_eq!(mode_formula(1, MAJOR).join(" "), "1 2 ♭3 4 5 6 ♭7");
        assert_eq!(mode_formula(3, MAJOR).join(" "), "1 2 3 ♯4 5 6 7");
        assert_eq!(mode_formula(6, MAJOR).join(" "), "1 ♭2 ♭3 4 ♭5 ♭6 ♭7");
        assert_eq!(mode_formula(0, MELODIC_MINOR).join(" "), "1 2 ♭3 4 5 6 7");
        assert_eq!(mode_formula(3, MELODIC_MINOR).join(" "), "1 2 3 ♯4 5 6 ♭7");
        assert_eq!(mode_formula(6, MELODIC_MINOR).join(" "), "1 ♭2 ♭3 ♭4 ♭5 ♭6 ♭7");
    }

    #[test]
    fn chords() {
        assert_eq!(chord_degrees(1, true), [1, 3, 5, 0]);
        assert_eq!(Quality::from_offsets(&[4, 7, 10]).symbol(), "7");
        assert_eq!(ascending_midi(&[2, 5, 9, 0], 48), [50, 53, 57, 60]);
        assert_eq!(roman(1, MAJOR, Quality::Min7), "iim7");
        assert_eq!(roman(2, MELODIC_MINOR, Quality::AugMaj7), "♭IIImaj7♯5");
        assert_eq!(roman(0, MELODIC_MINOR, Quality::MinMaj7), "imMaj7");
    }

    #[test]
    fn octaves() {
        let bs = Spelled { letter: 6, acc: 1 };
        assert_eq!(octave_of(bs, 60), 3); // B♯3 sounds like C4
        assert_eq!(octave_of(Spelled { letter: 0, acc: 0 }, 60), 4);
    }
}
