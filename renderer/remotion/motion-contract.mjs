const fail = (message) => { throw new Error(`Invalid motion input: ${message}`); };

function safeAsset(path, prefix, extensions) {
  if (typeof path !== 'string' || !path.startsWith(prefix) || path.includes('\\\\') || path.includes(':') || /[\r\n\0]/.test(path)) return false;
  const parts = path.split('/');
  if (parts.some((part) => !part || part === '.' || part === '..')) return false;
  return extensions.some((ext) => path.toLowerCase().endsWith(ext));
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
  return {durationInFrames: t.duration_frames};
}
