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


test('motion primitive snapshots validate', () => {
  const x = valid();
  x.subtitles = [
    {scene_id: 's1', utterance_index: 0, start_frame: 0, end_frame: 25, text: '第一句。'},
    {scene_id: 's1', utterance_index: 1, start_frame: 30, end_frame: 50, text: '第二句。'},
    {scene_id: 's2', utterance_index: 0, start_frame: 60, end_frame: 110, text: '第三句。'},
  ];
  x.scenes[0].steps = [
    {
      utterance_index: 0, start_frame: 0, duration_frames: 30,
      elements: [
        {type: 'stat', id: 'me', label: '你', value: '18:00', slot: 'left', emphasis: false},
        {type: 'stat', id: 'wang', label: '小王', value: '19:00', slot: 'right', emphasis: true},
        {type: 'relation', id: 'edge', from: 'me', to: 'wang', label: '相对变化', slot: 'center'},
      ],
    },
    {
      utterance_index: 1, start_frame: 30, duration_frames: 30,
      elements: [
        {type: 'matrix', id: 'payoff', title: '收益矩阵', headers: ['合作', '背叛'], rows: [['3,3', '0,5'], ['5,0', '1,1']], slot: 'center'},
        {type: 'formula', id: 'f', text: 'Sₜ₊₁ = F(Sₜ, Aₜ, Eₜ, εₜ)', highlight: 'Aₜ', note: '行动是输入之一', slot: 'bottom'},
      ],
    },
  ];
  assert.deepEqual(validateMotion(x), {durationInFrames: 120});
});

for (const [name, mutate] of [
  ['step gap', (x) => { x.scenes[0].steps[1].start_frame = 31; }],
  ['bad relation endpoint', (x) => { x.scenes[0].steps[0].elements[2].to = 'missing'; }],
  ['bad formula highlight', (x) => { x.scenes[0].steps[1].elements[1].highlight = 'missing'; }],
  ['bad matrix row', (x) => { x.scenes[0].steps[1].elements[0].rows[0].pop(); }],
  ['unknown primitive field', (x) => { x.scenes[0].steps[0].elements[0].script = 'no'; }],
]) {
  test(`reject primitive ${name}`, () => {
    const x = valid();
    x.subtitles = [
      {scene_id: 's1', utterance_index: 0, start_frame: 0, end_frame: 25, text: '第一句。'},
      {scene_id: 's1', utterance_index: 1, start_frame: 30, end_frame: 50, text: '第二句。'},
      {scene_id: 's2', utterance_index: 0, start_frame: 60, end_frame: 110, text: '第三句。'},
    ];
    x.scenes[0].steps = [
      {
        utterance_index: 0, start_frame: 0, duration_frames: 30,
        elements: [
          {type: 'stat', id: 'me', label: '你', value: '18:00', slot: 'left', emphasis: false},
          {type: 'stat', id: 'wang', label: '小王', value: '19:00', slot: 'right', emphasis: true},
          {type: 'relation', id: 'edge', from: 'me', to: 'wang', label: '变化', slot: 'center'},
        ],
      },
      {
        utterance_index: 1, start_frame: 30, duration_frames: 30,
        elements: [
          {type: 'matrix', id: 'payoff', title: '收益矩阵', headers: ['合作', '背叛'], rows: [['3,3', '0,5'], ['5,0', '1,1']], slot: 'center'},
          {type: 'formula', id: 'f', text: 'Sₜ₊₁ = F(Sₜ, Aₜ, Eₜ, εₜ)', highlight: 'Aₜ', note: '行动是输入之一', slot: 'bottom'},
        ],
      },
    ];
    mutate(x);
    assert.throws(() => validateMotion(x));
  });
}

test('reject step without a matching timed cue', () => {
  const x = valid();
  x.scenes[0].steps = [{utterance_index: 1, start_frame: 0, duration_frames: 60, elements: []}];
  assert.throws(() => validateMotion(x));
});
