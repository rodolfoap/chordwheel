# ChordWheel

A digital version of the paper chord wheel: two discs on a pin, the outer one
with the 12 notes and the inner one with the seven degrees of a major key.
Turn the inner wheel to put degree **I** on a note and read off every chord
and mode of that key.

**Live app:** https://rodolfoap.github.io/chordwheel/

- Drag the inner wheel (or use ⟲ ⟳) to change key; tap a degree to select
  its chord and mode and hear it.
- Diatonic seventh chords (Imaj7, iim7, iiim7, IVmaj7, V7, vim7, viim7♭5) or
  triads, each with its mode name (Ionian … Locrian).
- Note names with sharps, flats, or spelled the way the key does (Auto).
- Chromatic or circle-of-fifths layout.
- Major scale (with its natural minor) or melodic minor, each with its seven
  chords and modes (Dorian ♭2, Lydian dominant, Altered …).
- A guitar neck (default) or piano keyboard showing the scale, the selected
  chord and the tonic.
- A treble staff with the key signature, the selected mode and its chord, with
  note names and step numbers (1 2 ♭3 …).
- Sound: notes, chords and scales, with a tempo control.
- Chord details (all seven chords, their notes and roles) behind a toggle
  under the staff.
- A built-in manual with a worked example (Blue Bossa).
- Installable on Android as an app (PWA) and works offline.

## Run it locally

The built app is committed in `web/`, so no Rust toolchain is needed:

```sh
python3 -m http.server -d web 8000
```

Then open http://localhost:8000.

## Install on an Android tablet

Open the live app in Chrome, then ⋮ → **Install app** (or **Add to Home
screen**). Installing needs HTTPS, which GitHub Pages provides; a plain
`http://` address on your local network will run the app but not install it.

## Build

The music theory and the SVG drawing are Rust (`src/`), compiled to
WebAssembly; `web/app.js` handles input and `web/audio.js` the sound.

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.100
./build.sh      # writes web/pkg and bumps the offline cache version
cargo test      # theory and rendering tests
```

Commit `web/pkg` after building. Pushing to `main` runs the tests and
publishes `web/` to GitHub Pages (Settings → Pages → Source: GitHub Actions).

## Credits

Clef and accidental shapes come from the Noto Music font (SIL Open Font
License 1.1).
