const fail = (message) => { throw new Error(`Invalid motion input: ${message}`); };

function safeAsset(path, prefix, extensions) {
  if (typeof path !== 'string' || !path.startsWith(prefix) || path.includes('\\\\') || path.includes(':') || /[\r\n\0]/.test(path)) return false;
  const parts = path.split('/');
  if (parts.some((part) => !part || part === '.' || part === '..')) return false;
  return extensions.some((ext) => path.toLowerCase().endsWith(ext));
}

const slots = new Set(['top', 'left', 'center', 'right', 'bottom']);
const readable = (value, max, multiline = false) =>
  typeof value === 'string' && value.trim() && [...value].length <= max &&
  !(multiline ? /[\r\0]/ : /[\r\n\0]/).test(value);
const id = (value) => typeof value === 'string' && /^[A-Za-z0-9_-]{1,64}$/.test(value);
const exactKeys = (value, allowed) => Object.keys(value).every((key) => allowed.includes(key));

function validateElements(elements) {
  if (!Array.isArray(elements) || elements.length > 12) fail('step elements');
  const ids = new Set();
  const targets = new Set();
  for (const element of elements) {
    if (!element || typeof element !== 'object' || !id(element.id) || ids.has(element.id) || !slots.has(element.slot)) fail('element identity/slot');
    ids.add(element.id);
    switch (element.type) {
      case 'text':
        if (!exactKeys(element, ['type', 'id', 'text', 'slot', 'emphasis']) || !readable(element.text, 160, true) || typeof element.emphasis !== 'boolean') fail('text element');
        targets.add(element.id);
        break;
      case 'stat':
        if (!exactKeys(element, ['type', 'id', 'label', 'value', 'slot', 'emphasis']) || !readable(element.label, 48) || !readable(element.value, 48) || typeof element.emphasis !== 'boolean') fail('stat element');
        targets.add(element.id);
        break;
      case 'relation':
        if (!exactKeys(element, ['type', 'id', 'from', 'to', 'label', 'slot']) || !id(element.from) || !id(element.to) || (element.label !== null && !readable(element.label, 48))) fail('relation element');
        break;
      case 'matrix':
        if (!exactKeys(element, ['type', 'id', 'title', 'headers', 'rows', 'slot']) || !readable(element.title, 80) || !Array.isArray(element.headers) || element.headers.length < 2 || element.headers.length > 4 || !element.headers.every((cell) => readable(cell, 32)) || !Array.isArray(element.rows) || element.rows.length < 1 || element.rows.length > 4 || !element.rows.every((row) => Array.isArray(row) && row.length === element.headers.length && row.every((cell) => readable(cell, 40)))) fail('matrix element');
        targets.add(element.id);
        break;
      case 'formula':
        if (!exactKeys(element, ['type', 'id', 'text', 'highlight', 'note', 'slot']) || !readable(element.text, 180) || (element.highlight !== null && (!readable(element.highlight, 80) || !element.text.includes(element.highlight))) || (element.note !== null && !readable(element.note, 120, true))) fail('formula element');
        targets.add(element.id);
        break;
      default:
        fail('unknown element type');
    }
  }
  for (const element of elements) {
    if (element.type === 'relation' && (element.from === element.to || !targets.has(element.from) || !targets.has(element.to))) fail('relation endpoints');
  }
}

export function validateMotion(t) {
  if (t?.schema_version !== 1 || t.fps !== 30) fail('version/fps');
  if (!Number.isSafeInteger(t.width) || !Number.isSafeInteger(t.height) || t.width < 320 || t.height < 320 || t.width > 4096 || t.height > 4096) fail('dimensions');
  if (!Number.isSafeInteger(t.duration_frames) || t.duration_frames < 1 || t.duration_frames > 108000) fail('duration');
  if (typeof t.title !== 'string' || !t.title.trim() || [...t.title].length > 200) fail('title');
  if (t.audio !== 'audio/narration.wav') fail('audio path');
  if (!Array.isArray(t.scenes) || t.scenes.length < 1 || t.scenes.length > 80) fail('scenes');
  if (!Array.isArray(t.subtitles) || t.subtitles.length < 1) fail('subtitles');

  const ids = new Set();
  const byId = new Map();
  let frame = 0;
  for (const scene of t.scenes) {
    if (typeof scene.id !== 'string' || !/^[A-Za-z0-9_-]{1,64}$/.test(scene.id) || ids.has(scene.id)) fail('scene id');
    ids.add(scene.id);
    if (scene.start_frame !== frame || !Number.isSafeInteger(scene.duration_frames) || scene.duration_frames < 1) fail('scene sequence');
    if (!safeAsset(scene.image, 'images/', ['.png', '.jpg', '.jpeg', '.webp'])) fail('scene image path');
    if (typeof scene.screen_text !== 'string' || [...scene.screen_text].length > 400 || /[\r\0]/.test(scene.screen_text)) fail('screen text');
    if (scene.steps !== undefined) {
      if (!Array.isArray(scene.steps) || scene.steps.length < 1 || scene.steps.length > 40) fail('steps');
      let expectedStart = scene.start_frame;
      let previousUtterance = -1;
      for (const [stepIndex, step] of scene.steps.entries()) {
        if (!step || typeof step !== 'object' || !exactKeys(step, ['utterance_index', 'start_frame', 'duration_frames', 'elements'])) fail('step fields');
        if (!Number.isSafeInteger(step.utterance_index) || step.utterance_index < 0 || (stepIndex === 0 && step.utterance_index !== 0) || step.utterance_index <= previousUtterance) fail('step utterance');
        if (step.start_frame !== expectedStart || !Number.isSafeInteger(step.duration_frames) || step.duration_frames < 1) fail('step timing');
        validateElements(step.elements);
        previousUtterance = step.utterance_index;
        expectedStart += step.duration_frames;
      }
      if (expectedStart !== scene.start_frame + scene.duration_frames) fail('step coverage');
    }
    byId.set(scene.id, scene);
    frame += scene.duration_frames;
  }
  if (frame !== t.duration_frames) fail('scene coverage');

  let previousEnd = 0;
  const cueKeys = new Set();
  for (const cue of t.subtitles) {
    const scene = byId.get(cue.scene_id);
    if (!scene || !Number.isSafeInteger(cue.utterance_index) || cue.utterance_index < 0) fail('subtitle scene/index');
    if (!Number.isSafeInteger(cue.start_frame) || !Number.isSafeInteger(cue.end_frame) || cue.end_frame <= cue.start_frame || cue.start_frame < previousEnd || cue.end_frame > t.duration_frames) fail('subtitle timing');
    const sceneEnd = scene.start_frame + scene.duration_frames;
    if (cue.start_frame < scene.start_frame || cue.end_frame > sceneEnd) fail('subtitle outside scene');
    if (typeof cue.text !== 'string' || !cue.text.trim() || [...cue.text].length > 160 || /[\r\n]/.test(cue.text)) fail('subtitle text');
    const key = `${cue.scene_id}:${cue.utterance_index}`;
    if (cueKeys.has(key)) fail('duplicate subtitle');
    cueKeys.add(key);
    previousEnd = cue.end_frame;
  }
  for (const scene of t.scenes) {
    for (const step of scene.steps ?? []) {
      if (!cueKeys.has(`${scene.id}:${step.utterance_index}`)) fail('step without matching subtitle cue');
    }
  }
  return {durationInFrames: t.duration_frames};
}
