// A small Web Audio synth with a soft, piano-like tone.

let ctx = null;
let out = null;

function context() {
  if (!ctx) {
    ctx = new (window.AudioContext || window.webkitAudioContext)();
    const comp = ctx.createDynamicsCompressor();
    comp.threshold.value = -18;
    comp.ratio.value = 4;
    out = ctx.createGain();
    out.gain.value = 0.5;
    out.connect(comp).connect(ctx.destination);
  }
  if (ctx.state === 'suspended') ctx.resume();
  return ctx;
}

const freq = (midi) => 440 * Math.pow(2, (midi - 69) / 12);

/** Play one MIDI note `delay` seconds from now for `dur` seconds. */
export function note(midi, delay = 0, dur = 0.9, vel = 0.32) {
  const c = context();
  const t = c.currentTime + delay + 0.01;
  const g = c.createGain();
  const lp = c.createBiquadFilter();
  lp.type = 'lowpass';
  lp.frequency.setValueAtTime(Math.min(6000, freq(midi) * 6), t);
  lp.frequency.exponentialRampToValueAtTime(Math.max(400, freq(midi) * 2), t + dur);
  g.gain.setValueAtTime(0, t);
  g.gain.linearRampToValueAtTime(vel, t + 0.008);
  g.gain.exponentialRampToValueAtTime(vel * 0.35, t + 0.25);
  g.gain.exponentialRampToValueAtTime(0.0001, t + dur + 0.35);
  lp.connect(g).connect(out);
  for (const [type, mult, level] of [['triangle', 1, 1], ['sine', 2, 0.25], ['sine', 0.5, 0.2]]) {
    const o = c.createOscillator();
    const og = c.createGain();
    o.type = type;
    o.frequency.value = freq(midi) * mult;
    og.gain.value = level;
    o.connect(og).connect(lp);
    o.start(t);
    o.stop(t + dur + 0.4);
  }
}

/** Strum a chord: notes slightly spread, optionally with the root an octave down. */
export function chord(midis, delay = 0, dur = 1.2, bass = false) {
  midis.forEach((m, i) => note(m, delay + i * 0.03, dur, 0.22));
  if (bass) note(midis[0] - 12, delay, dur, 0.28);
}
