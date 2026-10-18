import init, { Wheel, ring_radii } from './pkg/chordwheel.js';
import * as audio from './audio.js';

const MAJOR = [0, 2, 4, 5, 7, 9, 11];
const MODES = ['Ionian', 'Dorian', 'Phrygian', 'Lydian', 'Mixolydian', 'Aeolian', 'Locrian'];
const STORE = 'chordwheel-settings';

await init();
const w = new Wheel();
const [R_HUB, R_INNER] = ring_radii();

const $ = (id) => document.getElementById(id);
const wheelEl = $('wheel');
const staffEl = $('staff');
const infoEl = $('info');
const esc = (s) => String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/"/g, '&quot;');

let drag = 0;          // extra rotation of the inner wheel while dragging, in degrees
let info = null;       // latest info from Rust
let bpm = 100;
let playingProg = null;
let playId = 0;
let timers = [];

// ---------- settings ----------

function load() {
  try {
    const s = JSON.parse(localStorage.getItem(STORE) || 'null') || {};
    w.key = s.key ?? 0;
    w.degree = s.degree ?? 0;
    w.accidentals = s.acc ?? 0;
    w.sevenths = s.sevenths ?? true;
    w.fifths = s.fifths ?? false;
    bpm = s.bpm ?? 100;
  } catch { /* storage unavailable: use defaults */ }
  fromHash();
}

// The address keeps the wheel's position (e.g. #k=3&d=5&a=2) so a setting
// can be bookmarked or linked to, as the manual does.
function fromHash() {
  const h = new URLSearchParams(location.hash.slice(1));
  const num = (k) => (h.has(k) && !Number.isNaN(Number(h.get(k))) ? Number(h.get(k)) : null);
  if (num('k') !== null) w.key = num('k');
  if (num('d') !== null) w.degree = num('d');
  if (num('a') !== null) w.accidentals = num('a');
  if (num('s') !== null) w.sevenths = num('s') === 1;
  if (num('f') !== null) w.fifths = num('f') === 1;
}

function save() {
  try {
    localStorage.setItem(STORE, JSON.stringify({
      key: w.key, degree: w.degree, acc: w.accidentals,
      sevenths: w.sevenths, fifths: w.fifths, bpm,
    }));
  } catch { /* ignore */ }
  const hash = `#k=${w.key}&d=${w.degree}&a=${w.accidentals}&s=${w.sevenths ? 1 : 0}&f=${w.fifths ? 1 : 0}`;
  if (location.hash !== hash) history.replaceState(null, '', hash);
}

// ---------- rendering ----------

function renderWheel() {
  wheelEl.innerHTML = w.wheel_svg(drag);
}

function renderInfo() {
  const c = info.chords[info.chord];
  const rows = info.chords.map((ch) => `
    <tr data-deg="${ch.degree}" class="${ch.degree === info.chord ? 'sel' : ''}">
      <td class="rn">${esc(ch.roman)}</td>
      <td class="nm">${esc(ch.name)}</td>
      <td class="nt">${esc(ch.notes.join(' '))}</td>
      <td><span class="badge ${ch.function.toLowerCase()}">${ch.function}</span></td>
      <td><button aria-label="Play ${esc(ch.name)}">▶</button></td>
    </tr>`).join('');
  const progs = info.progressions.map((p, i) => `
    <li class="${playingProg === i ? 'playing' : ''}">
      <div class="pt">
        <div><span class="pn">${esc(p.name)}</span> <span class="pc">${esc(p.roman)}</span></div>
        <div>${esc(p.chords)}</div>
        <div class="pd">${esc(p.description)}</div>
      </div>
      <button data-prog="${i}" aria-label="Play ${esc(p.name)} progression">▶</button>
    </li>`).join('');
  infoEl.innerHTML = `
    <div class="title"><h2>${esc(info.mode)}</h2><span class="chordname">${esc(c.name)}</span></div>
    <p>${esc(info.mode_description)}</p>
    <div class="facts">
      <span>Parent key <b>${esc(info.key)}</b> (${esc(info.signature)})</span>
      <span>Relative minor <b>${esc(info.relative_minor)}</b></span>
      <span>${esc(c.name)} is a <b>${esc(c.quality)}</b>: ${esc(c.notes.join(' '))}</span>
    </div>
    <h3>Chords in ${esc(info.key)}</h3>
    <table class="chords"><tbody>${rows}</tbody></table>
    <h3>Try a progression in ${esc(info.key)}</h3>
    <ul class="prog">${progs}</ul>`;
}

function renderControls() {
  const tonic = $('tonic');
  tonic.innerHTML = Array.from({ length: 12 }, (_, pc) =>
    `<option value="${pc}">${esc(w.name_of(pc))}</option>`).join('');
  tonic.value = String(w.tonic());
  const mode = $('mode');
  if (!mode.options.length) {
    mode.innerHTML = MODES.map((m, i) => `<option value="${i}">${m}</option>`).join('');
  }
  mode.value = String(w.degree);
  const seg = (id, v) => $(id).querySelectorAll('button').forEach((b) => {
    b.setAttribute('role', 'radio');
    b.setAttribute('aria-checked', String(b.dataset.v === String(v)));
  });
  seg('acc', w.accidentals);
  seg('size', w.sevenths ? 1 : 0);
  seg('layout', w.fifths ? 1 : 0);
  $('bpm').value = String(bpm);
  $('bpm-val').textContent = `${bpm}`;
  $('staff-title').textContent = `${info.mode}: ${info.scale.join(' ')}`;
}

function render() {
  renderWheel();
  staffEl.innerHTML = w.staff_svg();
  info = JSON.parse(w.info_json());
  renderInfo();
  renderControls();
  save();
}

// ---------- actions ----------

function selectDegree(d, play = true) {
  w.degree = d;
  render();
  if (play) playChord();
}

function animateDrag(to, done) {
  const from = drag;
  const t0 = performance.now();
  const dur = 140;
  const step = (t) => {
    const k = Math.min(1, (t - t0) / dur);
    drag = from + (to - from) * (1 - Math.pow(1 - k, 3));
    renderWheel();
    if (k < 1) requestAnimationFrame(step);
    else done();
  };
  requestAnimationFrame(step);
}

function turn(steps) {
  animateDrag(steps * 30, () => {
    drag = 0;
    w.rotate(steps);
    render();
  });
}

function snap() {
  const steps = Math.round(drag / 30);
  animateDrag(steps * 30, () => {
    drag = 0;
    if (steps) w.rotate(steps);
    render();
  });
}

function flash(selector, ms) {
  staffEl.querySelectorAll(selector).forEach((n) => {
    n.classList.add('playing');
    setTimeout(() => n.classList.remove('playing'), ms);
  });
}

function stop() {
  playId++;
  timers.forEach(clearTimeout);
  timers = [];
  $('stop').hidden = true;
  if (playingProg !== null) {
    playingProg = null;
    renderInfo();
  }
}

function later(ms, fn) {
  const id = playId;
  timers.push(setTimeout(() => { if (id === playId) fn(); }, ms));
}

function playChord() {
  const midi = w.chord_midi(w.degree);
  audio.chord(midi, 0, 1.4);
  flash('.chord .note', 900);
}

function playScale() {
  stop();
  $('stop').hidden = false;
  const beat = 60000 / bpm / 2;
  const up = info.scale_midi;
  const seq = up.concat(up.slice(0, -1).reverse());
  seq.forEach((m, i) => later(i * beat, () => {
    audio.note(m, 0, beat / 1000 * 1.8);
    flash(`:scope > svg > .note[data-midi="${m}"]`, beat * 0.9);
  }));
  later(seq.length * beat, stop);
}

function playProgression(i) {
  stop();
  $('stop').hidden = false;
  const p = info.progressions[i];
  const bar = 60000 / bpm * 2;
  p.degrees.forEach((d, k) => later(k * bar, () => {
    playingProg = i;
    w.degree = d;
    render();
    audio.chord(w.chord_midi(d), 0, bar / 1000 * 0.95, true);
  }));
  later(p.degrees.length * bar, stop);
}

// ---------- wheel input ----------

function wheelPoint(e) {
  const r = wheelEl.getBoundingClientRect();
  return { x: (e.clientX - r.left) / r.width * 400 - 200, y: (e.clientY - r.top) / r.height * 400 - 200 };
}
const angleOf = (p) => Math.atan2(p.x, -p.y) * 180 / Math.PI;

let ptr = null;

wheelEl.addEventListener('pointerdown', (e) => {
  const p = wheelPoint(e);
  const r = Math.hypot(p.x, p.y);
  ptr = { start: p, r, last: angleOf(p), moved: false, target: e.target };
  if (r <= R_INNER) wheelEl.setPointerCapture(e.pointerId);
});

wheelEl.addEventListener('pointermove', (e) => {
  if (!ptr || ptr.r > R_INNER) return;
  const p = wheelPoint(e);
  if (!ptr.moved && Math.hypot(p.x - ptr.start.x, p.y - ptr.start.y) < 6) return;
  if (Math.hypot(p.x, p.y) < 12) return; // too close to the pin to read an angle
  ptr.moved = true;
  wheelEl.classList.add('dragging');
  const a = angleOf(p);
  let d = a - ptr.last;
  if (d > 180) d -= 360;
  if (d < -180) d += 360;
  ptr.last = a;
  drag += d;
  renderWheel();
});

function endPointer() {
  if (!ptr) return;
  const { moved, target } = ptr;
  ptr = null;
  wheelEl.classList.remove('dragging');
  if (moved) return snap();
  const deg = target.closest?.('[data-deg]');
  const pc = target.closest?.('[data-pc]');
  if (deg) selectDegree(Number(deg.dataset.deg));
  else if (pc) audio.note(60 + Number(pc.dataset.pc), 0, 1);
  else if (target.classList?.contains('hub')) playChord();
}
wheelEl.addEventListener('pointerup', endPointer);
wheelEl.addEventListener('pointercancel', () => { if (ptr?.moved) { ptr = null; wheelEl.classList.remove('dragging'); snap(); } else ptr = null; });

// ---------- other input ----------

$('cw').addEventListener('click', () => turn(1));
$('ccw').addEventListener('click', () => turn(-1));

$('tonic').addEventListener('change', (e) => {
  w.key = (Number(e.target.value) - MAJOR[w.degree] + 12) % 12;
  render();
});
$('mode').addEventListener('change', (e) => {
  w.set_mode_keep_tonic(Number(e.target.value));
  render();
});

const onSeg = (id, fn) => $(id).addEventListener('click', (e) => {
  const b = e.target.closest('button');
  if (!b) return;
  fn(Number(b.dataset.v));
  render();
});
onSeg('acc', (v) => { w.accidentals = v; });
onSeg('size', (v) => { w.sevenths = v === 1; });
onSeg('layout', (v) => { w.fifths = v === 1; });

$('bpm').addEventListener('input', (e) => {
  bpm = Number(e.target.value);
  $('bpm-val').textContent = `${bpm}`;
  save();
});

$('play-scale').addEventListener('click', playScale);
$('play-chord').addEventListener('click', playChord);
$('stop').addEventListener('click', stop);

infoEl.addEventListener('click', (e) => {
  const prog = e.target.closest('[data-prog]');
  if (prog) return playProgression(Number(prog.dataset.prog));
  const row = e.target.closest('tr[data-deg]');
  if (row) selectDegree(Number(row.dataset.deg));
});

staffEl.addEventListener('click', (e) => {
  if (e.target.closest('.chord')) return playChord();
  const n = e.target.closest('[data-midi]');
  if (n) {
    audio.note(Number(n.dataset.midi), 0, 1);
    n.classList.add('playing');
    setTimeout(() => n.classList.remove('playing'), 500);
  }
});

document.addEventListener('keydown', (e) => {
  if (e.target.closest('select, input') || e.ctrlKey || e.metaKey || e.altKey) return;
  if (e.key === 'ArrowRight') turn(1);
  else if (e.key === 'ArrowLeft') turn(-1);
  else if (e.key >= '1' && e.key <= '7') selectDegree(Number(e.key) - 1);
  else if (e.key === ' ' && !e.target.closest('button')) { e.preventDefault(); playChord(); }
  else return;
});

window.addEventListener('hashchange', () => { fromHash(); render(); });

const manual = $('manual');
$('open-manual').addEventListener('click', () => manual.showModal());
$('close-manual').addEventListener('click', () => manual.close());
manual.addEventListener('click', (e) => { if (e.target === manual) manual.close(); });
// Links inside the manual set the wheel and close it.
window.addEventListener('message', (e) => {
  if (e.origin !== location.origin || typeof e.data?.wheel !== 'string') return;
  manual.close();
  location.hash = e.data.wheel;
});

load();
render();

if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('sw.js').catch(() => {});
}
