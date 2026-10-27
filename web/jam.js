// A one-minute swing jam over the selected chord and scale: walking bass,
// piano comping, brushes-style drums and a guitar improvising lines and
// triads. Self-contained: it has its own audio context and only reads the
// harmony through the `harmony` callback, so the rest of the app never
// depends on it.

const LOOKAHEAD = 0.15; // seconds of audio scheduled ahead of time
const TICK_MS = 25;

const freq = (m) => 440 * Math.pow(2, (m - 69) / 12);
const pick = (arr) => arr[Math.floor(Math.random() * arr.length)];
const chance = (p) => Math.random() < p;

// ---------- instruments ----------

function makeOut(ctx) {
  const comp = ctx.createDynamicsCompressor();
  comp.threshold.value = -16;
  comp.ratio.value = 3;
  const out = ctx.createGain();
  out.gain.value = 0.55;
  out.connect(comp).connect(ctx.destination);
  return out;
}

function noise(ctx) {
  const len = ctx.sampleRate;
  const buf = ctx.createBuffer(1, len, ctx.sampleRate);
  const d = buf.getChannelData(0);
  for (let i = 0; i < len; i++) d[i] = Math.random() * 2 - 1;
  return buf;
}

function tone(ctx, out, { midi, t, dur, vel, type = 'triangle', cutoff = 2000, attack = 0.005, partials = [[1, 1]] }) {
  const g = ctx.createGain();
  const lp = ctx.createBiquadFilter();
  lp.type = 'lowpass';
  lp.frequency.setValueAtTime(cutoff, t);
  lp.frequency.exponentialRampToValueAtTime(Math.max(200, cutoff / 3), t + dur);
  g.gain.setValueAtTime(0, t);
  g.gain.linearRampToValueAtTime(vel, t + attack);
  g.gain.exponentialRampToValueAtTime(vel * 0.3, t + dur * 0.5);
  g.gain.exponentialRampToValueAtTime(0.0001, t + dur + 0.08);
  lp.connect(g).connect(out);
  for (const [mult, level] of partials) {
    const o = ctx.createOscillator();
    const og = ctx.createGain();
    o.type = type;
    o.frequency.value = freq(midi) * mult;
    og.gain.value = level;
    o.connect(og).connect(lp);
    o.start(t);
    o.stop(t + dur + 0.1);
  }
}

const bass = (ctx, out, midi, t, dur) =>
  tone(ctx, out, { midi, t, dur, vel: 0.5, type: 'sine', cutoff: 900, attack: 0.01, partials: [[1, 1], [2, 0.35], [3, 0.1]] });

const piano = (ctx, out, midi, t, dur) =>
  tone(ctx, out, { midi, t, dur, vel: 0.11, type: 'triangle', cutoff: 2600, partials: [[1, 1], [2, 0.3], [4, 0.05]] });

const guitar = (ctx, out, midi, t, dur, vel = 0.17) =>
  tone(ctx, out, { midi, t, dur, vel, type: 'sawtooth', cutoff: 1500, attack: 0.004, partials: [[1, 0.5], [2, 0.12]] });

function hit(ctx, out, buf, t, { vel, type, f, q = 1, decay }) {
  const src = ctx.createBufferSource();
  src.buffer = buf;
  const filt = ctx.createBiquadFilter();
  filt.type = type;
  filt.frequency.value = f;
  filt.Q.value = q;
  const g = ctx.createGain();
  g.gain.setValueAtTime(vel, t);
  g.gain.exponentialRampToValueAtTime(0.0001, t + decay);
  src.connect(filt).connect(g).connect(out);
  src.start(t, Math.random() * 0.5);
  src.stop(t + decay + 0.05);
}

const ride = (ctx, out, buf, t, vel) => hit(ctx, out, buf, t, { vel, type: 'bandpass', f: 7200, q: 2.5, decay: 0.45 });
const hihat = (ctx, out, buf, t) => hit(ctx, out, buf, t, { vel: 0.2, type: 'highpass', f: 8000, decay: 0.05 });
const snare = (ctx, out, buf, t, vel) => hit(ctx, out, buf, t, { vel, type: 'bandpass', f: 1800, q: 0.8, decay: 0.12 });

function kick(ctx, out, t, vel) {
  const o = ctx.createOscillator();
  const g = ctx.createGain();
  o.frequency.setValueAtTime(110, t);
  o.frequency.exponentialRampToValueAtTime(45, t + 0.12);
  g.gain.setValueAtTime(vel, t);
  g.gain.exponentialRampToValueAtTime(0.0001, t + 0.2);
  o.connect(g).connect(out);
  o.start(t);
  o.stop(t + 0.25);
}

// ---------- musical choices ----------

/** Nearest MIDI note with pitch class `pc` to `near`, kept within [lo, hi]. */
function nearest(pc, near, lo, hi) {
  let best = null;
  for (let m = lo; m <= hi; m++) {
    if (m % 12 === pc && (best === null || Math.abs(m - near) < Math.abs(best - near))) best = m;
  }
  return best ?? near;
}

/** A walking bass bar: root, two chord or scale tones, then a note leading back to the root. */
function walkBar(h, prev) {
  const root = nearest(h.chord[0], prev, 31, 45);
  const pool = h.chord.concat(h.scale);
  const notes = [root];
  let cur = root;
  for (let i = 0; i < 2; i++) {
    cur = nearest(pick(pool), cur + pick([-4, -2, 2, 4, 5]), 28, 48);
    notes.push(cur);
  }
  // Chromatic or diatonic approach to next bar's root.
  notes.push(chance(0.6) ? root + pick([-1, 1]) : nearest(pick(h.scale), root + 2, 28, 48));
  return notes;
}

/** Piano voicing: the chord's 3rd and 7th (or 5th) plus the root's colour, around middle C. */
function voicing(h) {
  const tones = h.chord.slice(1);
  let m = 52;
  return tones.map((pc) => (m = nearest(pc, m + 3, 50, 70)));
}

// Comping rhythms, in beats from the start of the bar (0.67 is a swung "and").
const COMPS = [[0, 1.67], [1.67, 3.67], [0.67, 2], [1, 3], [2.67], [0, 2.67, 3.67], [1.67]];

/** One bar of guitar: a scale line in swung eighths, or triads from the scale. */
function guitarBar(h, prev) {
  const scale = [];
  for (let m = 55; m <= 81; m++) if (h.scale.includes(m % 12)) scale.push(m);
  const events = [];
  if (chance(0.3)) {
    // Triads built on scale notes, stacked in thirds.
    for (const beat of pick([[0, 2], [1.67, 3], [0.67, 2.67], [0]])) {
      const i = Math.floor(Math.random() * (scale.length - 5));
      events.push({ beat, notes: [scale[i], scale[i + 2], scale[i + 4]], dur: 0.8 });
    }
    return { events, last: prev };
  }
  let idx = Math.max(0, scale.findIndex((m) => m >= prev));
  const restBeats = chance(0.4) ? Math.floor(Math.random() * 4) : -1;
  for (let e = 0; e < 8; e++) {
    const beat = Math.floor(e / 2) + (e % 2 ? 0.67 : 0);
    if (Math.floor(beat) === restBeats || chance(0.12)) continue;
    idx = Math.min(scale.length - 1, Math.max(0, idx + pick([-2, -1, -1, 1, 1, 2, 3, -3])));
    let note = scale[idx];
    // Land on chord tones on the beat.
    if (e % 2 === 0 && !h.chord.includes(note % 12) && chance(0.6)) {
      const up = scale.slice(idx).find((m) => h.chord.includes(m % 12));
      if (up) note = up;
    }
    events.push({ beat, notes: [note], dur: e % 2 ? 0.3 : 0.5, accent: e % 2 === 1 });
  }
  const last = events.length ? events[events.length - 1].notes[0] : prev;
  return { events, last };
}

// ---------- the jam ----------

/**
 * Start a swing jam. `harmony()` returns { chord: [pitch classes, root first],
 * scale: [pitch classes] } and is read every bar, so changing the chord on the
 * wheel changes what the band plays. Returns a function that stops the jam.
 */
export function startJam({ harmony, bpm = 120, seconds = 60, onBar = () => {}, onEnd = () => {} }) {
  const ctx = new (window.AudioContext || window.webkitAudioContext)();
  const out = makeOut(ctx);
  const buf = noise(ctx);
  const beat = 60 / Math.max(80, Math.min(220, bpm));
  const bars = Math.max(4, Math.round(seconds / (beat * 4)));
  let bar = 0;
  let barStart = ctx.currentTime + 0.1;
  let bassPrev = 38;
  let guitarPrev = 67;
  let stopped = false;

  const at = (b) => barStart + b * beat;

  function scheduleBar() {
    let h;
    try {
      h = harmony();
    } catch {
      h = null;
    }
    if (!h || !h.chord?.length || !h.scale?.length) h = { chord: [0, 4, 7, 11], scale: [0, 2, 4, 5, 7, 9, 11] };
    const final = bar === bars - 1;
    const intro = bar === 0;

    // Drums: ride "ding, ding-a, ding, ding-a", hi-hat on 2 and 4, feathered kick.
    for (const [b, v] of [[0, 0.32], [1, 0.4], [1.67, 0.22], [2, 0.32], [3, 0.4], [3.67, 0.22]]) {
      if (final && b > 0) break;
      ride(ctx, out, buf, at(b), v);
    }
    for (let b = 0; b < 4 && !final; b++) {
      kick(ctx, out, at(b), 0.18);
      if (b % 2) hihat(ctx, out, buf, at(b));
    }
    if (!final && chance(0.35)) snare(ctx, out, buf, at(pick([1.67, 2.67, 3.67])), 0.12);

    // Bass.
    const walk = walkBar(h, bassPrev);
    if (final) {
      bass(ctx, out, walk[0], at(0), beat * 3);
    } else {
      walk.forEach((m, i) => bass(ctx, out, m, at(i), beat * 0.9));
      bassPrev = walk[0];
    }

    // Piano.
    const v = voicing(h);
    const rhythm = final ? [0] : pick(COMPS);
    for (const b of rhythm) v.forEach((m) => piano(ctx, out, m, at(b) + Math.random() * 0.01, final ? beat * 3 : beat * 0.7));

    // Guitar improvises after a one-bar intro.
    if (final) {
      const top = nearest(h.chord[h.chord.length - 1], 72, 64, 79);
      guitar(ctx, out, top, at(0), beat * 3, 0.15);
    } else if (!intro) {
      const g = guitarBar(h, guitarPrev);
      for (const e of g.events) {
        e.notes.forEach((m, i) => guitar(ctx, out, m, at(e.beat) + i * 0.02, e.dur * beat, e.accent ? 0.2 : 0.16));
      }
      guitarPrev = g.last;
    }

    onBar(bar, bars);
    bar++;
    barStart += beat * 4;
  }

  const timer = setInterval(() => {
    if (stopped) return;
    while (bar < bars && barStart < ctx.currentTime + LOOKAHEAD + beat * 4) scheduleBar();
    if (bar >= bars && ctx.currentTime > barStart - beat * 0.5) stop();
  }, TICK_MS);

  function stop() {
    if (stopped) return;
    stopped = true;
    clearInterval(timer);
    out.gain.setTargetAtTime(0, ctx.currentTime, 0.1);
    setTimeout(() => ctx.close().catch(() => {}), 600);
    onEnd();
  }

  return stop;
}
