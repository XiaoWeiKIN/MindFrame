import {readFile, access, mkdtemp, rm, rename} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import {resolve, join, dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
import {bundle} from '@remotion/bundler';
import {ensureBrowser, renderMedia, renderStill, selectComposition} from '@remotion/renderer';
import {validateMotion} from './motion-contract.mjs';

const home = dirname(fileURLToPath(import.meta.url));
const browserExecutable = process.env.REMOTION_BROWSER_EXECUTABLE;

async function main() {
  if (browserExecutable) await access(browserExecutable);
  else await ensureBrowser();
  if (process.argv[2] === '--check') {
    console.log('MindFrame Motion renderer and browser: ready');
    return;
  }

  const [projectArg, scaleArg = '1'] = process.argv.slice(2);
  if (!projectArg) throw new Error('Usage: node motion-render.mjs PROJECT [SCALE]');
  const scale = Number(scaleArg);
  if (!Number.isFinite(scale) || scale < 0.1 || scale > 1) throw new Error('scale must be 0.1..1');

  const project = resolve(projectArg);
  const inputProps = JSON.parse(await readFile(join(project, 'motion-input.json'), 'utf8'));
  const {durationInFrames} = validateMotion(inputProps);
  const assets = join(project, 'assets');
  await access(join(assets, inputProps.audio));
  for (const scene of inputProps.scenes) await access(join(assets, scene.image));

  const work = await mkdtemp(join(tmpdir(), 'mindframe-motion-'));
  const pendingVideo = join(project, 'motion.pending.mp4');
  const pendingCover = join(project, 'cover.pending.png');
  try {
    const serveUrl = await bundle({entryPoint: join(home, 'src/motion.tsx'), publicDir: assets, outDir: join(work, 'bundle')});
    const common = {serveUrl, inputProps, browserExecutable};
    const composition = await selectComposition({...common, id: 'MindFrameMotion'});
    await renderMedia({...common, composition, codec: 'h264', audioCodec: 'aac', pixelFormat: 'yuv420p', imageFormat: 'png', colorSpace: 'bt709', crf: 20, concurrency: 2, scale, outputLocation: pendingVideo});
    await renderStill({...common, composition, frame: Math.min(20, durationInFrames - 1), imageFormat: 'png', scale, output: pendingCover});
    await rename(pendingVideo, join(project, 'motion.mp4'));
    await rename(pendingCover, join(project, 'cover.png'));
    console.log(`Rendered MindFrame Motion: ${durationInFrames} frames, scale=${scale}`);
  } finally {
    await rm(work, {recursive: true, force: true});
    await rm(pendingVideo, {force: true});
    await rm(pendingCover, {force: true});
  }
}

main().catch((error) => { console.error(error.message); process.exitCode = 1; });
