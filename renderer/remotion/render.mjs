import {readFile, access, mkdtemp, rm, rename} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import {resolve, join, dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
import {bundle} from '@remotion/bundler';
import {ensureBrowser, renderMedia, renderStill, selectComposition} from '@remotion/renderer';
import {dimensions, validateTimeline} from './contract.mjs';

const home = dirname(fileURLToPath(import.meta.url));
const browserExecutable = process.env.REMOTION_BROWSER_EXECUTABLE;

async function main() {
  if (browserExecutable) await access(browserExecutable);
  else await ensureBrowser();
  if (process.argv[2] === '--check') {
    console.log('Remotion dependencies and browser: ready');
    return;
  }
  const [projectArg, preset, scaleArg = '1'] = process.argv.slice(2);
  if (!projectArg) throw new Error('Usage: node render.mjs PROJECT bilibili|douyin [SCALE]');
  const scale = Number(scaleArg);
  if (!Number.isFinite(scale) || scale < 0.1 || scale > 1) throw new Error('scale must be 0.1..1');
  const project = resolve(projectArg);
  const timeline = JSON.parse(await readFile(join(project, 'render-input.json'), 'utf8'));
  const {durationInFrames} = validateTimeline(timeline);
  const assets = join(project, 'assets');
  for (const clip of timeline.clips) await access(join(assets, clip.audio));
  for (const scene of timeline.storyboard.scenes) {
    if (scene.visual.type === 'image') await access(join(assets, `${scene.id}.png`));
  }
  const inputProps = {...timeline, ...dimensions(preset)};
  const work = await mkdtemp(join(tmpdir(), 'mindframe-render-'));
  const pendingVideo = join(project, `${preset}.pending.mp4`);
  const pendingCover = join(project, `${preset}.pending.png`);
  try {
    // Only generated assets are served. Source notes and API configuration are not bundled.
    const serveUrl = await bundle({entryPoint: join(home, 'src/index.tsx'), publicDir: assets, outDir: join(work, 'bundle')});
    const common = {serveUrl, inputProps, browserExecutable};
    const composition = await selectComposition({...common, id: 'MindFrame'});
    await renderMedia({...common, composition, codec: 'h264', audioCodec: 'aac', pixelFormat: 'yuv420p', crf: 20, concurrency: 2, scale, outputLocation: pendingVideo});
    await renderStill({...common, composition, frame: Math.min(20, durationInFrames - 1), imageFormat: 'png', scale, output: pendingCover});
    await rename(pendingVideo, join(project, `${preset}.mp4`));
    await rename(pendingCover, join(project, `cover-${preset}.png`));
    console.log(`Rendered ${preset}: ${durationInFrames} frames, scale=${scale}`);
  } finally {
    await rm(work, {recursive: true, force: true});
    await rm(pendingVideo, {force: true});
    await rm(pendingCover, {force: true});
  }
}

main().catch((error) => { console.error(error.message); process.exitCode = 1; });
