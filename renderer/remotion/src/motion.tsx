import React from 'react';
import {AbsoluteFill, Audio, Composition, Img, Sequence, getInputProps, interpolate, registerRoot, staticFile, useCurrentFrame, useVideoConfig} from 'remotion';
import type {MotionProps, MotionScene} from './motion-types';

const Background: React.FC<{scene: MotionScene; globalStart: number; total: number}> = ({scene, globalStart, total}) => {
  const local = useCurrentFrame();
  const global = globalStart + local;
  const phase = total <= 1 ? 0 : global / (total - 1);
  const scale = interpolate(phase, [0, 1], [1.025, 1.065], {extrapolateLeft: 'clamp', extrapolateRight: 'clamp'});
  const x = Math.sin(phase * Math.PI * 2) * 0.7;
  const y = Math.cos(phase * Math.PI * 2) * 0.45;
  return <AbsoluteFill style={{overflow: 'hidden'}}>
    <Img src={staticFile(scene.image)} style={{width: '100%', height: '100%', objectFit: 'cover', transform: `scale(${scale}) translate(${x}%, ${y}%)`, filter: 'brightness(0.52) saturate(0.82)'}} />
    <AbsoluteFill style={{background: 'linear-gradient(180deg, rgba(3,12,22,.30) 0%, rgba(3,12,22,.12) 42%, rgba(3,12,22,.48) 100%)'}} />
    <AbsoluteFill style={{background: 'radial-gradient(circle at 50% 42%, rgba(7,21,33,.06) 0%, rgba(7,21,33,.18) 52%, rgba(2,8,15,.42) 100%)'}} />
  </AbsoluteFill>;
};

const Emphasis: React.FC<{text: string; duration: number}> = ({text, duration}) => {
  const frame = useCurrentFrame();
  const {width, height} = useVideoConfig();
  if (!text.trim()) return null;
  const portrait = height > width;
  const enter = interpolate(frame, [0, Math.min(10, Math.max(1, duration - 1))], [0, 1], {extrapolateLeft: 'clamp', extrapolateRight: 'clamp'});
  const lines = text.split('\n').map((line) => line.trim()).filter(Boolean);
  const [headline, ...rest] = lines;
  return <div style={{
    position: 'absolute',
    top: portrait ? '27%' : '24%',
    left: portrait ? '9%' : '12%',
    right: portrait ? '9%' : '12%',
    textAlign: 'center',
    opacity: enter,
    transform: `translateY(${(1 - enter) * 18}px)`,
    color: '#F1EEE7',
    textShadow: '0 2px 18px rgba(0,0,0,.45)',
  }}>
    <div style={{fontFamily: '"Noto Serif CJK SC","Songti SC",serif', fontWeight: 700, fontSize: portrait ? 82 : 84, lineHeight: 1.2, letterSpacing: 2}}>{headline}</div>
    {rest.length > 0 && <div style={{marginTop: portrait ? 34 : 28, fontFamily: '"Noto Sans CJK SC","PingFang SC",sans-serif', fontSize: portrait ? 44 : 40, lineHeight: 1.5, whiteSpace: 'pre-wrap', color: '#E8E4DC'}}>{rest.join('\n')}</div>}
    <div style={{width: portrait ? 120 : 110, height: 3, margin: portrait ? '42px auto 0' : '34px auto 0', background: '#CDA66A', opacity: .9}} />
  </div>;
};

const MotionVideo: React.FC<MotionProps> = ({title, audio, scenes, subtitles, duration_frames}) => {
  const {width, height} = useVideoConfig();
  const portrait = height > width;
  return <AbsoluteFill style={{background: '#071521', color: '#F1EEE7', fontFamily: '"Noto Sans CJK SC","PingFang SC","Microsoft YaHei",sans-serif'}}>
    {scenes.map((scene) => <Sequence key={scene.id} from={scene.start_frame} durationInFrames={scene.duration_frames}>
      <Background scene={scene} globalStart={scene.start_frame} total={duration_frames} />
      <Emphasis text={scene.screen_text} duration={scene.duration_frames} />
    </Sequence>)}

    <div style={{position: 'absolute', top: portrait ? 86 : 52, left: portrait ? 72 : 88, right: portrait ? 72 : 88, display: 'flex', justifyContent: 'space-between', gap: 40, fontSize: portrait ? 22 : 20, color: 'rgba(241,238,231,.60)', letterSpacing: 3}}>
      <span>MINDFRAME</span>
      <span style={{overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap'}}>{title}</span>
    </div>

    {subtitles.map((cue) => <Sequence key={`${cue.scene_id}-${cue.utterance_index}`} from={cue.start_frame} durationInFrames={cue.end_frame - cue.start_frame}>
      <div style={{
        position: 'absolute',
        left: portrait ? '8%' : '15%',
        right: portrait ? '8%' : '15%',
        bottom: portrait ? '10%' : '7%',
        textAlign: 'center',
        fontSize: portrait ? 46 : 40,
        lineHeight: 1.45,
        fontWeight: 600,
        color: '#F8F6F1',
        textShadow: '0 2px 12px rgba(0,0,0,.75)',
        padding: portrait ? '18px 24px' : '12px 20px',
        borderRadius: 12,
        background: 'rgba(2,8,15,.28)',
      }}>{cue.text}</div>
    </Sequence>)}

    <Audio src={staticFile(audio)} />
  </AbsoluteFill>;
};

const Root: React.FC = () => {
  const props = getInputProps<MotionProps>();
  if (!props.scenes?.length || !props.subtitles?.length) throw new Error('Use mindframe motion with a prepared chat-material project');
  return <Composition id="MindFrameMotion" component={MotionVideo} defaultProps={props} width={props.width} height={props.height} fps={props.fps} durationInFrames={props.duration_frames} />;
};

registerRoot(Root);
