"""Real chat-material Motion render using local eSpeak narration. No model API calls."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import struct
import subprocess
import tempfile
import wave
import zlib

ROOT = Path(__file__).resolve().parents[1]

SOURCE = """# Motion fixture
A small workplace game can change when one coworker stays later.
A payoff matrix is a compact way to compare strategic outcomes.
State transitions depend on action, environment, and disturbance.
"""


def png_fixture(width: int = 640, height: int = 360) -> bytes:
    def chunk(kind: bytes, data: bytes) -> bytes:
        return struct.pack('!I', len(data)) + kind + data + struct.pack('!I', zlib.crc32(kind + data) & 0xffffffff)
    rows = []
    for y in range(height):
        row = bytearray([0])
        for x in range(width):
            row.extend((7 + x * 8 // width, 21 + y * 10 // height, 33 + x * 12 // width))
        rows.append(bytes(row))
    return b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('!2I5B', width, height, 8, 2, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(b''.join(rows), 6)) + chunk(b'IEND', b'')


def run(*args: object, cwd: Path = ROOT) -> subprocess.CompletedProcess[str]:
    result = subprocess.run([str(x) for x in args], cwd=cwd, env=os.environ.copy(), capture_output=True, text=True)
    if result.returncode:
        raise AssertionError(result.stdout + result.stderr)
    return result


def speech_clip(text: str, path: Path) -> tuple[wave._wave_params, bytes, int]:
    run('espeak', '-v', 'en', '-s', '155', '-w', path, text)
    with wave.open(str(path), 'rb') as reader:
        params = reader.getparams()
        frames = reader.readframes(reader.getnframes())
        return params, frames, reader.getnframes()


def make_voice(out: Path, texts: list[str]) -> tuple[list[tuple[int, int]], int]:
    pieces = []
    params = None
    cursor_frames = 0
    cues = []
    with tempfile.TemporaryDirectory() as tmp:
        for i, text in enumerate(texts):
            p, data, nframes = speech_clip(text, Path(tmp) / f'{i}.wav')
            if params is None:
                params = p
            else:
                assert (p.nchannels, p.sampwidth, p.framerate, p.comptype) == (params.nchannels, params.sampwidth, params.framerate, params.comptype)
            start_ms = cursor_frames * 1000 // p.framerate
            cursor_frames += nframes
            end_ms = cursor_frames * 1000 // p.framerate
            cues.append((start_ms, end_ms))
            pieces.append(data)
    assert params is not None
    with wave.open(str(out), 'wb') as writer:
        writer.setparams(params)
        for data in pieces:
            writer.writeframes(data)
    return cues, cursor_frames * 1000 // params.framerate


def check_video(path: Path, dims: tuple[int, int], expected_seconds: float) -> None:
    probe = json.loads(subprocess.check_output(['ffprobe', '-v', 'error', '-show_streams', '-show_format', '-of', 'json', str(path)]))
    video = next(s for s in probe['streams'] if s['codec_type'] == 'video')
    audio = next(s for s in probe['streams'] if s['codec_type'] == 'audio')
    assert (video['width'], video['height']) == dims
    assert video['codec_name'] == 'h264' and video['pix_fmt'] == 'yuv420p'
    assert video['r_frame_rate'] == '30/1' and audio['codec_name'] == 'aac'
    assert abs(float(probe['format']['duration']) - expected_seconds) < 0.25
    subprocess.run(['ffmpeg', '-v', 'error', '-i', str(path), '-f', 'null', '-'], check=True, capture_output=True)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', type=Path, default=ROOT / 'target/debug/mindframe')
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    binary = args.binary.resolve()

    source = out / 'source.md'
    source.write_text(SOURCE)
    project = out / 'project'
    run(binary, 'init', source, '--out', project, '--preset', 'douyin')

    bundle = out / 'bundle'
    bundle.mkdir()
    background = bundle / 'background.png'
    background.write_bytes(png_fixture())
    texts = [
        'Suppose you and Xiao Wang both leave at six.',
        'One day Xiao Wang starts leaving at seven.',
        'The boss now gives Xiao Wang a relative advantage.',
        'Then you also leave at seven, so the relative advantage disappears.',
        'A payoff matrix compares what each strategy combination returns.',
        'A state transition depends on action, environment, and disturbance.',
    ]
    cue_ranges, duration_ms = make_voice(bundle / 'voice.wav', texts)

    board = {
        'schema_version': 1,
        'title': 'Motion primitives fixture',
        'summary': 'Synthetic strategic example rendered with fixed motion primitives.',
        'key_points': [
            {'text': 'Workplace game', 'sources': [{'line_start': 2, 'line_end': 2, 'quote': 'A small workplace game can change when one coworker stays later.'}]},
            {'text': 'Payoff matrix', 'sources': [{'line_start': 3, 'line_end': 3, 'quote': 'A payoff matrix is a compact way to compare strategic outcomes.'}]},
            {'text': 'State transition', 'sources': [{'line_start': 4, 'line_end': 4, 'quote': 'State transitions depend on action, environment, and disturbance.'}]},
        ],
        'scenes': [
            {'id': 'game', 'narration': texts[:4], 'point_refs': [1], 'visual': {'type': 'key_point', 'title': '相对优势', 'body': 'A synthetic workplace game.'}, 'transition': 'fade'},
            {'id': 'matrix', 'narration': [texts[4]], 'point_refs': [2], 'visual': {'type': 'key_point', 'title': '收益矩阵', 'body': 'Compare strategy combinations.'}, 'transition': 'fade'},
            {'id': 'formula', 'narration': [texts[5]], 'point_refs': [3], 'visual': {'type': 'key_point', 'title': '状态转移', 'body': 'Action is only one input.'}, 'transition': 'fade'},
        ],
    }
    assets = {
        'schema_version': 1,
        'images': [
            {'scene_id': 'game', 'file': 'background.png', 'screen_text': ''},
            {'scene_id': 'matrix', 'file': 'background.png', 'screen_text': ''},
            {'scene_id': 'formula', 'file': 'background.png', 'screen_text': ''},
        ],
        'audio': 'voice.wav',
        'cues': [
            {'scene_id': 'game', 'utterance_index': 0, 'text': texts[0], 'start_ms': cue_ranges[0][0], 'end_ms': cue_ranges[0][1]},
            {'scene_id': 'game', 'utterance_index': 1, 'text': texts[1], 'start_ms': cue_ranges[1][0], 'end_ms': cue_ranges[1][1]},
            {'scene_id': 'game', 'utterance_index': 2, 'text': texts[2], 'start_ms': cue_ranges[2][0], 'end_ms': cue_ranges[2][1]},
            {'scene_id': 'game', 'utterance_index': 3, 'text': texts[3], 'start_ms': cue_ranges[3][0], 'end_ms': cue_ranges[3][1]},
            {'scene_id': 'matrix', 'utterance_index': 0, 'text': texts[4], 'start_ms': cue_ranges[4][0], 'end_ms': cue_ranges[4][1]},
            {'scene_id': 'formula', 'utterance_index': 0, 'text': texts[5], 'start_ms': cue_ranges[5][0], 'end_ms': cue_ranges[5][1]},
        ],
    }
    motion = {
        'schema_version': 1,
        'scenes': [
            {
                'scene_id': 'game',
                'steps': [
                    {'utterance_index': 0, 'elements': [
                        {'type': 'text', 'id': 'setup', 'text': '假设公司只有你和小王', 'slot': 'top', 'emphasis': False},
                        {'type': 'stat', 'id': 'you', 'label': '你', 'value': '18:00', 'slot': 'left', 'emphasis': False},
                        {'type': 'stat', 'id': 'wang', 'label': '小王', 'value': '18:00', 'slot': 'right', 'emphasis': False},
                    ]},
                    {'utterance_index': 1, 'elements': [
                        {'type': 'stat', 'id': 'you', 'label': '你', 'value': '18:00', 'slot': 'left', 'emphasis': False},
                        {'type': 'stat', 'id': 'wang', 'label': '小王', 'value': '19:00 ↑', 'slot': 'right', 'emphasis': True},
                    ]},
                    {'utterance_index': 2, 'elements': [
                        {'type': 'stat', 'id': 'you', 'label': '你', 'value': '18:00', 'slot': 'left', 'emphasis': False},
                        {'type': 'stat', 'id': 'wang', 'label': '小王', 'value': '19:00', 'slot': 'right', 'emphasis': True},
                        {'type': 'stat', 'id': 'boss', 'label': '老板评价', 'value': '小王 +1', 'slot': 'top', 'emphasis': True},
                        {'type': 'relation', 'id': 'boss-edge', 'from': 'boss', 'to': 'wang', 'label': '更努力', 'slot': 'center'},
                    ]},
                    {'utterance_index': 3, 'elements': [
                        {'type': 'stat', 'id': 'you', 'label': '你', 'value': '19:00', 'slot': 'left', 'emphasis': False},
                        {'type': 'stat', 'id': 'wang', 'label': '小王', 'value': '19:00', 'slot': 'right', 'emphasis': False},
                        {'type': 'text', 'id': 'result', 'text': '两个人都多上班 1 小时\n相对优势重新归零', 'slot': 'center', 'emphasis': True},
                    ]},
                ],
            },
            {
                'scene_id': 'matrix',
                'steps': [
                    {'utterance_index': 0, 'elements': [
                        {'type': 'matrix', 'id': 'payoff', 'title': '收益矩阵', 'headers': ['合作', '背叛'], 'rows': [['3,3', '0,5'], ['5,0', '1,1']], 'slot': 'center'},
                    ]},
                ],
            },
            {
                'scene_id': 'formula',
                'steps': [
                    {'utterance_index': 0, 'elements': [
                        {'type': 'formula', 'id': 'state', 'text': 'Sₜ₊₁ = F(Sₜ, Aₜ, Eₜ, εₜ)', 'highlight': 'Aₜ', 'note': '行动只是输入之一', 'slot': 'center'},
                    ]},
                ],
            },
        ],
    }
    (bundle / 'storyboard.json').write_text(json.dumps(board, ensure_ascii=False, indent=2))
    (bundle / 'assets.json').write_text(json.dumps(assets, ensure_ascii=False, indent=2))
    (bundle / 'motion.json').write_text(json.dumps(motion, ensure_ascii=False, indent=2))
    run(binary, 'import', project, '--from', bundle)

    expected = duration_ms / 1000
    results = {}
    for preset, dims in [('bilibili', (480, 270)), ('douyin', (270, 480))]:
        target = out / preset
        run(binary, 'motion', project, '--out', target, '--preset', preset, '--renderer', ROOT / 'renderer/remotion', '--scale', '0.25')
        assert (target / 'subtitles.srt').is_file()
        input_data = json.loads((target / 'motion-input.json').read_text())
        assert input_data['audio'] == 'audio/narration.wav'
        assert len(input_data['scenes'][0]['steps']) == 4
        assert input_data['scenes'][0]['steps'][1]['elements'][1]['value'] == '19:00 ↑'
        assert input_data['scenes'][1]['steps'][0]['elements'][0]['type'] == 'matrix'
        assert input_data['scenes'][2]['steps'][0]['elements'][0]['type'] == 'formula'
        assert (target / 'motion.json').is_file()
        assert (target / 'assets/images/001-game.png').read_bytes() == background.read_bytes()
        assert (target / 'assets/images/002-matrix.png').read_bytes() == background.read_bytes()
        assert (target / 'assets/images/003-formula.png').read_bytes() == background.read_bytes()
        check_video(target / 'motion.mp4', dims, expected)
        assert (target / 'cover.png').stat().st_size > 1000
        results[preset] = {'dimensions': dims, 'duration_seconds': expected, 'decode': 'passed'}

    (out / 'verification.json').write_text(json.dumps({
        'audio': 'local eSpeak fixture with exact concatenated utterance boundaries; not a natural-voice quality test',
        'background': 'one synthetic static PNG reused across all scenes with renderer-created subtle motion',
        'presets': results,
        'alignment': 'sentence boundaries derived from separately rendered fixture utterances',
    }, indent=2))
    print('Motion primitives: text/stat/relation/matrix/formula snapshots, real WAV, reviewed cues, both MP4 layouts and full decode passed.')


if __name__ == '__main__':
    main()
