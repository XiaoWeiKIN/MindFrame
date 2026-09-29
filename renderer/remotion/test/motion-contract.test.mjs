import test from 'node:test';
import assert from 'node:assert/strict';
import {validateMotion} from '../motion-contract.mjs';

const valid = () => ({
  schema_version: 1,
  fps: 30,
  width: 1920,
  height: 1080,
  duration_frames: 120,
  title: 'Fixture',
  audio: 'audio/narration.wav',
  scenes: [
    {id: 's1', start_frame: 0, duration_frames: 60, image: 'images/001-s1.png', screen_text: '路径依赖\nPath Dependence'},
    {id: 's2', start_frame: 60, duration_frames: 60, image: 'images/002-s2.webp', screen_text: '反身性'},
  ],
  subtitles: [
    {scene_id: 's1', utterance_index: 0, start_frame: 0, end_frame: 50, text: '第一句。'},
    {scene_id: 's2', utterance_index: 0, start_frame: 60, end_frame: 110, text: '第二句。'},
  ],
});

test('valid motion input covers exact duration', () => {
  assert.deepEqual(validateMotion(valid()), {durationInFrames: 120});
});

for (const [name, mutate] of [
  ['bad version', (x) => x.schema_version = 2],
  ['bad audio path', (x) => x.audio = '../voice.wav'],
  ['scene gap', (x) => x.scenes[1].start_frame = 61],
  ['scene traversal', (x) => x.scenes[0].image = 'images/../x.png'],
  ['duplicate scene', (x) => x.scenes[1].id = 's1'],
  ['subtitle overlap', (x) => x.subtitles[1].start_frame = 49],
  ['subtitle outside scene', (x) => x.subtitles[0].end_frame = 61],
  ['duplicate subtitle key', (x) => { x.subtitles[1].scene_id = 's1'; x.subtitles[1].utterance_index = 0; x.subtitles[1].start_frame = 50; x.subtitles[1].end_frame = 55; }],
  ['oversize screen text', (x) => x.scenes[0].screen_text = 'x'.repeat(401)],
]) {
  test(`reject ${name}`, () => {
    const x = valid();
    mutate(x);
    assert.throws(() => validateMotion(x));
  });
}
