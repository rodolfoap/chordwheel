// A small Web Audio synth with a soft, piano-like tone.
//
// Each note is a single oscillator with a custom waveform (instead of one
// oscillator per harmonic), which keeps the audio graph small enough for
// tablets to render without dropouts, and the master chain leaves headroom so
// stacked notes never clip.

let ctx = null;
let out = null;
let wave = null;

function context() {
  if (!ctx) {
    const AC = window.AudioContext || window.webkitAudioContext;
    try {
      ctx = new AC({ latencyHint: 'balanced' });
    } catch {
      ctx = new AC();
    }
    const comp = ctx.createDynamicsCompressor();
    comp.threshold.value = -14;
    comp.knee.value = 12;
    comp.ratio.value = 4;
    comp.attack.value = 0.01;
    comp.release.value = 0.2;
    out = ctx.createGain();
    out.gain.value = 0.35;
    out.connect(comp).connect(ctx.destination);
    // Fundamental, a soft octave and a touch of the 3rd harmonic.
    wave = ctx.createPeriodicWave(new Float32Array([0, 0, 0, 0, 0]), new Float32Array([0, 1, 0.3, 0.08, 0.04]));
  }
  if (ctx.state === 'suspended') ctx.resume();
  return ctx;
}

const freq = (midi) => 440 * Math.pow(2, (midi - 69) / 12);

/** Play one MIDI note `delay` seconds from now for `dur` seconds. */
export function note(midi, delay = 0, dur = 0.9, vel = 0.32) {
  const c = context();
  const t = c.currentTime + delay + 0.02;
  const end = t + dur;
  const g = c.createGain();
  const lp = c.createBiquadFilter();
  lp.type = 'lowpass';
  lp.frequency.setValueAtTime(Math.min(6000, freq(midi) * 6), t);
  lp.frequency.exponentialRampToValueAtTime(Math.max(400, freq(midi) * 2), end);
  g.gain.setValueAtTime(0, t);
  g.gain.linearRampToValueAtTime(vel, t + 0.012);
  g.gain.setTargetAtTime(vel * 0.35, t + 0.012, 0.12);
  // Smooth release, then silence well before the oscillator stops.
  g.gain.setTargetAtTime(0, end, 0.08);
  const o = c.createOscillator();
  o.setPeriodicWave(wave);
  o.frequency.value = freq(midi);
  o.connect(lp).connect(g).connect(out);
  o.start(t);
  o.stop(end + 0.6);
}

/** Strum a chord: notes slightly spread, optionally with the root an octave down. */
export function chord(midis, delay = 0, dur = 1.2, bass = false) {
  midis.forEach((m, i) => note(m, delay + i * 0.03, dur, 0.22));
  if (bass) note(midis[0] - 12, delay, dur, 0.28);
}
