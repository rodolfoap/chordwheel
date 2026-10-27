// A small Web Audio synth with a soft, piano-like tone.
//
// Each note is rendered once into a buffer with an OfflineAudioContext and
// cached; playing it is then a single buffer source on the live context. The
// live audio thread does almost no work, so slow tablets don't crackle.

let ctx = null;
let out = null;
const cache = new Map();

function context() {
  if (!ctx) {
    const AC = window.AudioContext || window.webkitAudioContext;
    try {
      ctx = new AC({ latencyHint: 'balanced' });
    } catch {
      ctx = new AC();
    }
    out = ctx.createGain();
    out.gain.value = 0.35;
    out.connect(ctx.destination);
  }
  if (ctx.state === 'suspended') ctx.resume();
  return ctx;
}

const freq = (midi) => 440 * Math.pow(2, (midi - 69) / 12);

/** Render one note (`dur` seconds plus its release) into an AudioBuffer. */
function render(midi, dur, vel) {
  const rate = ctx.sampleRate;
  const off = new OfflineAudioContext(1, Math.ceil((dur + 0.6) * rate), rate);
  // Fundamental, a soft octave and a touch of the 3rd harmonic.
  const wave = off.createPeriodicWave(new Float32Array(5), new Float32Array([0, 1, 0.3, 0.08, 0.04]));
  const t = 0.002;
  const end = t + dur;
  const g = off.createGain();
  const lp = off.createBiquadFilter();
  lp.type = 'lowpass';
  lp.frequency.setValueAtTime(Math.min(6000, freq(midi) * 6), t);
  lp.frequency.exponentialRampToValueAtTime(Math.max(400, freq(midi) * 2), end);
  g.gain.setValueAtTime(0, 0);
  g.gain.linearRampToValueAtTime(vel, t + 0.012);
  g.gain.setTargetAtTime(vel * 0.35, t + 0.012, 0.12);
  g.gain.setTargetAtTime(0, end, 0.08);
  const o = off.createOscillator();
  o.setPeriodicWave(wave);
  o.frequency.value = freq(midi);
  o.connect(lp).connect(g).connect(off.destination);
  o.start(0);
  return off.startRendering();
}

function buffer(midi, dur, vel) {
  const key = `${midi}:${dur.toFixed(2)}:${vel}`;
  if (!cache.has(key)) {
    if (cache.size > 300) cache.clear();
    cache.set(key, render(midi, dur, vel));
  }
  return cache.get(key);
}

/** Play one MIDI note `delay` seconds from now for `dur` seconds. */
export function note(midi, delay = 0, dur = 0.9, vel = 0.32) {
  const c = context();
  const when = c.currentTime + delay + 0.02;
  buffer(midi, dur, vel).then((buf) => {
    const src = c.createBufferSource();
    src.buffer = buf;
    src.connect(out);
    src.start(Math.max(when, c.currentTime));
  }).catch((err) => console.error('Note failed', err));
}

/** Strum a chord: notes slightly spread, optionally with the root an octave down. */
export function chord(midis, delay = 0, dur = 1.2, bass = false) {
  midis.forEach((m, i) => note(m, delay + i * 0.03, dur, 0.22));
  if (bass) note(midis[0] - 12, delay, dur, 0.28);
}
