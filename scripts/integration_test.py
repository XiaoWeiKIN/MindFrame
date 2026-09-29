"""Loopback provider-contract test + real FFmpeg/Remotion integration. No paid API calls."""
from __future__ import annotations

import argparse
import base64
import copy
import json
import os
from pathlib import Path
import struct
import subprocess
import tempfile
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import zlib

ROOT = Path(__file__).resolve().parents[1]
SOURCE = (ROOT / 'examples/demo/source.md').read_text()
BOARD = json.loads((ROOT / 'examples/demo/storyboard.json').read_text())
# These three scenes exercise renderers; this is test content, not a live model result.
BOARD['scenes'] += [
    {'id': 'quote', 'narration': ['口播可以改变表达，但不能改变原意。'], 'point_refs': [2], 'visual': {'type': 'quote', 'text': '口播稿把书面内容改写成便于听懂的句子，但不应改变原意。', 'source': '测试素材'}, 'transition': 'cut'},
    {'id': 'image', 'narration': ['配图服务于解释，不代替知识本身。'], 'point_refs': [2], 'visual': {'type': 'image', 'prompt': 'Test fixture, blue concept illustration'}, 'transition': 'fade'},
    {'id': 'code', 'narration': ['让口播、声音与画面共同组成视频。'], 'point_refs': [3], 'visual': {'type': 'code', 'language': 'text', 'code': '知识\n  → 口播\n  → 声音与画面\n  → 视频'}, 'transition': 'slide'},
]


def png_fixture() -> bytes:
    def chunk(kind: bytes, data: bytes) -> bytes:
        return struct.pack('!I', len(data)) + kind + data + struct.pack('!I', zlib.crc32(kind + data) & 0xffffffff)
    pixels = b''.join(b'\0' + b''.join(bytes((30 + x, 60 + y, 170)) for x in range(128)) for y in range(128))
    return b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('!2I5B', 128, 128, 8, 2, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(pixels)) + chunk(b'IEND', b'')


class Provider(BaseHTTPRequestHandler):
    mode = 'good'
    calls: list[str] = []

    def log_message(self, *_: object) -> None:
        pass

    def do_POST(self) -> None:
        Provider.calls.append(self.path)
        body = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
        if Provider.mode == 'unauthorized':
            self.send_response(401)
            self.end_headers()
            self.wfile.write(b'test-secret-must-not-be-logged')
            return
        if self.path == '/v1/chat/completions':
            assert body['response_format'] == {'type': 'json_object'}
            assert '1: # ' in body['messages'][1]['content']
            board = copy.deepcopy(BOARD)
            if Provider.mode == 'bad-source':
                board['key_points'][0]['sources'][0]['quote'] = 'invented quotation'
            data = json.dumps({'choices': [{'message': {'content': json.dumps(board, ensure_ascii=False)}}]}).encode()
        elif self.path == '/v1/audio/speech':
            assert body['response_format'] == 'wav'
            with tempfile.TemporaryDirectory() as tmp:
                target = Path(tmp) / 'speech.wav'
                subprocess.run(['espeak', '-v', 'zh', '-s', '165', '-w', str(target), '--stdin'], input=body['input'].encode(), check=True, capture_output=True)
                data = target.read_bytes()
        elif self.path == '/v1/images/generations':
            assert body['output_format'] == 'png'
            data = json.dumps({'data': [{'b64_json': base64.b64encode(png_fixture()).decode()}]}).encode()
        else:
            self.send_error(404)
            return
        self.send_response(200)
        self.send_header('Content-Length', str(len(data)))
        self.end_headers()
        self.wfile.write(data)


def invoke(binary: Path, *args: object, succeeds: bool = True) -> subprocess.CompletedProcess[str]:
    result = subprocess.run([str(binary), *map(str, args)], cwd=ROOT, env={**os.environ, 'MINDFRAME_TEST_KEY': 'test-only'}, capture_output=True, text=True)
    if succeeds and result.returncode:
        raise AssertionError(result.stdout + result.stderr)
    if not succeeds and result.returncode == 0:
        raise AssertionError('expected failure')
    return result


def check_media(project: Path) -> None:
    timeline = json.loads((project / 'timeline.json').read_text())
    expected = sum(clip['duration_frames'] for clip in timeline['clips']) / 30
    subtitles = (project / 'subtitles.srt').read_text().strip().split('\n\n')
    assert len(subtitles) == len(timeline['clips'])
    for preset, dimensions in [('bilibili', (480, 270)), ('douyin', (270, 480))]:
        path = project / f'{preset}.mp4'
        probe = json.loads(subprocess.check_output(['ffprobe', '-v', 'error', '-show_streams', '-show_format', '-of', 'json', str(path)]))
        video = next(s for s in probe['streams'] if s['codec_type'] == 'video')
        audio = next(s for s in probe['streams'] if s['codec_type'] == 'audio')
        assert (video['width'], video['height']) == dimensions
        assert video['codec_name'] == 'h264' and video['pix_fmt'] == 'yuv420p'
        assert video['r_frame_rate'] == '30/1' and audio['codec_name'] == 'aac'
        assert abs(float(probe['format']['duration']) - expected) < 0.15
        subprocess.run(['ffmpeg', '-v', 'error', '-i', str(path), '-f', 'null', '-'], check=True, capture_output=True)
        assert (project / f'cover-{preset}.png').stat().st_size > 1000
        print(f'{preset}: {video["width"]}x{video["height"]}, H.264/AAC, {expected:.3f}s, full decode passed')
    (project / 'verification.json').write_text(json.dumps({'provider_mode': 'loopback fixtures, NOT live APIs', 'frames': round(expected * 30), 'presets': ['bilibili', 'douyin'], 'decode': 'passed'}, indent=2))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', type=Path, default=ROOT / 'target/debug/mindframe')
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    binary = args.binary.resolve()
    server = ThreadingHTTPServer(('127.0.0.1', 0), Provider)
    worker = threading.Thread(target=server.serve_forever, daemon=True)
    worker.start()
    try:
        url = f'http://127.0.0.1:{server.server_port}/v1'
        config = out / 'test.toml'
        endpoint = f'base_url="{url}"\nmodel="test-fixture"\napi_key_env="MINDFRAME_TEST_KEY"\n'
        config.write_text('[llm]\n' + endpoint + '\n[tts]\nprovider="api"\nvoice="test"\n[tts.api]\n' + endpoint + '\n[image]\n' + endpoint)
        source = out / 'input.md'
        source.write_text(SOURCE)
        project = out / 'project'
        invoke(binary, 'plan', source, '--out', project, '--config', config)
        before = len(Provider.calls)
        invoke(binary, 'plan', source, '--out', project, '--config', config, succeeds=False)
        assert len(Provider.calls) == before, 'existing project must fail before a paid call'
        invoke(binary, 'produce', project, '--config', config, '--preset', 'both', '--scale', '0.25')
        check_media(project)
        assert Provider.calls.count('/v1/chat/completions') == 1
        assert Provider.calls.count('/v1/images/generations') == 1
        assert Provider.calls.count('/v1/audio/speech') == len(BOARD['scenes'])
        before = len(Provider.calls)
        invoke(binary, 'produce', project, '--config', config, succeeds=False)
        assert len(Provider.calls) == before, 'existing assets must not trigger paid regeneration'
        for mode in ['unauthorized', 'bad-source']:
            Provider.mode = mode
            result = invoke(binary, 'plan', source, '--out', out / mode, '--config', config, succeeds=False)
            assert 'test-secret-must-not-be-logged' not in result.stderr
            assert not (out / mode / 'storyboard.json').exists()
        Provider.mode = 'good'
        edited = copy.deepcopy(BOARD)
        edited['scenes'][0]['narration'] = ['已经修改的口播。']
        (project / 'storyboard.json').write_text(json.dumps(edited, ensure_ascii=False))
        result = invoke(binary, 'render', project, succeeds=False)
        assert 'storyboard changed' in result.stderr
        (project / 'storyboard.json').write_text(json.dumps(BOARD, ensure_ascii=False, indent=2))
        print('Provider shape, all six visuals, both MP4 layouts, fail-fast, redaction and stale-plan checks passed.')
    finally:
        server.shutdown()
        server.server_close()
        worker.join()


if __name__ == '__main__':
    main()
