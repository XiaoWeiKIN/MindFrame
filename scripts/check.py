"""One repository verification entrypoint. No silent skips for missing toolchains."""
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
commands = [
    ['cargo', 'clippy', '--workspace', '--all-targets', '--', '-D', 'warnings'],
    ['cargo', 'test', '--workspace'],
    ['npm', '--prefix', 'renderer/remotion', 'run', 'check'],
    ['npm', '--prefix', 'renderer/remotion', 'test'],
]
for command in commands:
    print('+', ' '.join(command), flush=True)
    subprocess.run(command, cwd=ROOT, check=True)
