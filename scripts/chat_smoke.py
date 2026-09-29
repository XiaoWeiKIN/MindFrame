"""Exercise the actual Rust CLI with clearly synthetic image/audio fixtures, never model APIs."""
import argparse
import csv
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import wave
import zlib


def png() -> bytes:
    def chunk(kind: bytes, payload: bytes) -> bytes:
        return struct.pack('>I', len(payload)) + kind + payload + struct.pack('>I', zlib.crc32(kind + payload))
    width, height = 18, 32
    pixels = (b'\x00' + b'\x20\x40\x60' * width) * height
    return b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', width, height, 8, 2, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(pixels)) + chunk(b'IEND', b'')


def run(binary: Path, *args: object, success: bool = True) -> None:
    result = subprocess.run([str(binary), *map(str, args)], env={'PATH': ''}, text=True, capture_output=True)
    if (result.returncode == 0) != success:
        raise AssertionError(result.stdout + result.stderr)
    print(result.stdout.strip())


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('--out', required=True, type=Path)
    parser.add_argument('--binary', type=Path, default=Path('target/debug/mindframe'))
    args = parser.parse_args()
    binary = args.binary.resolve()
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    source = out / 'source.md'
    source.write_text('# 验收素材\n素材需要与场景明确对应。\n', encoding='utf-8')
    bundle = out / 'authored-fixture'
    bundle.mkdir()
    image = png()
    (bundle / 'fixture.png').write_bytes(image)
    board = {'schema_version': 1, 'title': '素材交接验收', 'summary': '检查分镜与实际图片对应。', 'key_points': [{'text': '素材需要与场景明确对应。', 'sources': [{'line_start': 2, 'line_end': 2, 'quote': '素材需要与场景明确对应。'}]}], 'scenes': [{'id': 'scene-01', 'narration': ['素材需要与场景明确对应。'], 'point_refs': [1], 'visual': {'type': 'image', 'prompt': 'Synthetic test fixture, not a generated illustration'}, 'transition': 'fade'}]}
    assets = {'schema_version': 1, 'images': [{'scene_id': 'scene-01', 'file': 'fixture.png', 'screen_text': '标题,含"引号"'}]}
    (bundle / 'storyboard.json').write_text(json.dumps(board, ensure_ascii=False), encoding='utf-8')
    manifest = bundle / 'assets.json'
    manifest.write_text(json.dumps(assets, ensure_ascii=False), encoding='utf-8')
    project = out / 'project'
    run(binary, 'init', source, '--out', project)
    run(binary, 'import', project, '--from', bundle)
    run(binary, 'validate', project)
    export = out / 'jianying-untimed'
    run(binary, 'export', project, '--target', 'jianying', '--out', export)
    assert not (export / 'subtitles.srt').exists()
    assert not (export / 'source.md').exists()
    assert (export / 'images/001-scene-01.png').read_bytes() == image
    with (export / 'shot-list.csv').open(encoding='utf-8-sig', newline='') as stream:
        row = list(csv.DictReader(stream))[0]
        assert row['start'] == row['end'] == ''
        assert row['screen_text'] == assets['images'][0]['screen_text']
    run(binary, 'export', project, '--out', export, success=False)
    # A real silent WAV plus manually supplied cue tests timing contracts, not speech alignment.
    with wave.open(str(bundle / 'voice.wav'), 'wb') as audio:
        audio.setparams((1, 2, 8000, 0, 'NONE', 'not compressed'))
        audio.writeframes(b'\0\0' * 16000)
    assets['audio'] = 'voice.wav'
    assets['cues'] = [{'scene_id': 'scene-01', 'utterance_index': 0, 'text': board['scenes'][0]['narration'][0], 'start_ms': 123, 'end_ms': 1750}]
    manifest.write_text(json.dumps(assets, ensure_ascii=False), encoding='utf-8')
    timed = out / 'timed-project'
    run(binary, 'init', source, '--out', timed)
    run(binary, 'import', timed, '--from', bundle)
    run(binary, 'export', timed, '--out', out / 'jianying-timed')
    assert '00:00:00,123 --> 00:00:01,750' in (out / 'jianying-timed/subtitles.srt').read_text(encoding='utf-8')
    (out / 'verification.json').write_text(json.dumps({'actual_cli': str(binary.name), 'primary_path': 'passed', 'env': 'PATH empty; no API keys', 'fixture': 'synthetic PNG and silent WAV; NOT ChatGPT output or real speech', 'image_sha256': hashlib.sha256(image).hexdigest(), 'editor_ui_tested': False, 'speech_alignment_verified': False}, indent=2), encoding='utf-8')
    print('Chat material CLI smoke: passed')


if __name__ == '__main__':
    main()
