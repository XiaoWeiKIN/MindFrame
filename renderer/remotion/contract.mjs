// The renderer is an external boundary: reject malformed or unsafe timeline files.
export function validateTimeline(t) {
  const fail = (message) => { throw new Error(`Invalid timeline: ${message}`); };
  if (t?.schema_version !== 1 || t.fps !== 30 || t.storyboard?.schema_version !== 1) fail('version/fps');
  const board = t.storyboard;
  if (typeof board.title !== 'string' || !board.title.trim() || !Array.isArray(board.scenes) || board.scenes.length === 0 || board.scenes.length > 80) fail('storyboard');
  if (!Array.isArray(t.clips)) fail('clips');
  const fields = {title: ['text'], key_point: ['title', 'body'], image: ['prompt'], quote: ['text', 'source'], diagram: ['mermaid'], code: ['language', 'code']};
  const ids = new Set();
  let frame = 0;
  let index = 0;
  for (const [si, scene] of board.scenes.entries()) {
    if (typeof scene.id !== 'string' || !/^[a-zA-Z0-9_-]{1,64}$/.test(scene.id) || ids.has(scene.id)) fail('scene id');
    ids.add(scene.id);
    if (!['cut', 'fade', 'slide'].includes(scene.transition)) fail('transition');
    const required = fields[scene.visual?.type];
    if (!Array.isArray(required) || required.some((key) => typeof scene.visual[key] !== 'string' || !scene.visual[key].trim())) fail('visual');
    if (scene.visual.type === 'diagram' && (!/^(flowchart|graph) /.test(scene.visual.mermaid.trim()) || /%%\{|<|click /.test(scene.visual.mermaid))) fail('unsafe Mermaid');
    if (!Array.isArray(scene.narration) || scene.narration.length === 0 || scene.narration.length > 40) fail('narration');
    for (const [ui, text] of scene.narration.entries()) {
      if (typeof text !== 'string' || !text.trim() || [...text].length > 160 || /[\r\n]/.test(text)) fail('utterance');
      const clip = t.clips[index++];
      const name = `${String(si).padStart(3, '0')}-${String(ui).padStart(3, '0')}.wav`;
      if (!clip || clip.scene_index !== si || clip.utterance_index !== ui || clip.start_frame !== frame || !Number.isSafeInteger(clip.duration_frames) || clip.duration_frames < 1 || clip.audio !== name) fail('clip sequence or asset path');
      frame += clip.duration_frames;
    }
  }
  if (index !== t.clips.length || frame > 108000) fail('extra clips or duration over one hour');
  return {durationInFrames: frame};
}

export function dimensions(preset) {
  if (preset === 'bilibili') return {width: 1920, height: 1080};
  if (preset === 'douyin') return {width: 1080, height: 1920};
  throw new Error(`Unknown preset: ${preset}`);
}
