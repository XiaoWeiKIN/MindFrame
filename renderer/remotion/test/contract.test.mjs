import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {validateTimeline, dimensions} from '../contract.mjs';

const board = JSON.parse(readFileSync(new URL('../../../examples/demo/storyboard.json', import.meta.url), 'utf8'));
function fixture() {
  return {schema_version: 1, fps: 30, storyboard: structuredClone(board), clips: board.scenes.map((_, index) => ({scene_index: index, utterance_index: 0, start_frame: index * 31, duration_frames: 31, audio: `${String(index).padStart(3, '0')}-000.wav`}))};
}

test('valid ordered clips determine exact total frames', () => assert.deepEqual(validateTimeline(fixture()), {durationInFrames: 93}));
test('landscape and portrait are independent layouts', () => { assert.deepEqual(dimensions('bilibili'), {width: 1920, height: 1080}); assert.deepEqual(dimensions('douyin'), {width: 1080, height: 1920}); assert.throws(() => dimensions('unknown')); });
for (const [name, mutate] of [
  ['gap', t => t.clips[1].start_frame++],
  ['overlap', t => t.clips[1].start_frame--],
  ['fractional duration', t => t.clips[0].duration_frames = 1.2],
  ['zero duration', t => t.clips[0].duration_frames = 0],
  ['path traversal', t => t.clips[0].audio = '../secret.wav'],
  ['remote URL', t => t.clips[0].audio = 'https://example.com/audio.wav'],
  ['duplicate id', t => t.storyboard.scenes[1].id = t.storyboard.scenes[0].id],
  ['bad visual', t => t.storyboard.scenes[0].visual.type = 'javascript'],
  ['bad transition', t => t.storyboard.scenes[0].transition = 'arbitrary'],
  ['extra clip', t => t.clips.push(t.clips[0])],
]) test(`reject ${name}`, () => { const t = fixture(); mutate(t); assert.throws(() => validateTimeline(t)); });
