import React, {useEffect, useState} from 'react';
import {AbsoluteFill, Audio, Composition, Img, Sequence, cancelRender, continueRender, delayRender, getInputProps, interpolate, registerRoot, staticFile, useCurrentFrame, useVideoConfig} from 'remotion';
import mermaid from 'mermaid';
import type {Scene, VideoProps} from './types';

mermaid.initialize({startOnLoad: false, securityLevel: 'strict', theme: 'dark', fontFamily: 'Noto Sans CJK SC, sans-serif', flowchart: {htmlLabels: false}});

const Diagram: React.FC<{code: string; id: string}> = ({code, id}) => {
  const [handle] = useState(() => delayRender('Render Mermaid diagram'));
  const [image, setImage] = useState<string>();
  useEffect(() => {
    mermaid.render(`mf_${id}`, code).then(({svg}) => {
      setImage(`data:image/svg+xml;charset=utf-8,${encodeURIComponent(svg)}`);
      continueRender(handle);
    }).catch((error: unknown) => cancelRender(error instanceof Error ? error : new Error(String(error))));
  }, [code, id, handle]);
  return image ? <Img src={image} style={{width: '100%', height: '100%', objectFit: 'contain'}} /> : null;
};

const SceneCard: React.FC<{scene: Scene; duration: number; index: number}> = ({scene, duration, index}) => {
  const frame = useCurrentFrame();
  const {width, height} = useVideoConfig();
  const portrait = height > width;
  const progress = interpolate(frame, [0, Math.max(1, Math.min(12, duration - 1))], [0, 1], {extrapolateLeft: 'clamp', extrapolateRight: 'clamp'});
  const opacity = scene.transition === 'cut' ? 1 : progress;
  const translate = scene.transition === 'slide' ? (1 - progress) * 65 : 0;
  const visual = scene.visual;
  const titleStyle: React.CSSProperties = {fontSize: portrait ? 76 : 94, fontWeight: 750, lineHeight: 1.2, margin: 0, letterSpacing: -2, overflowWrap: 'anywhere'};
  let content: React.ReactNode;
  switch (visual.type) {
    case 'title':
      content = <><div style={{color: '#7dd3fc', fontSize: 25, letterSpacing: 6, marginBottom: 34}}>KNOWLEDGE / IN FOCUS</div><h1 style={titleStyle}>{visual.text}</h1><div style={{width: 100, height: 6, background: '#fbbf24', marginTop: 46}} /></>;
      break;
    case 'key_point':
      content = <><h2 style={titleStyle}>{visual.title}</h2><p style={{fontSize: portrait ? 43 : 48, lineHeight: 1.65, color: '#cbd5e1', whiteSpace: 'pre-wrap'}}>{visual.body}</p></>;
      break;
    case 'image':
      content = <Img src={staticFile(`${scene.id}.png`)} style={{width: '100%', height: '100%', objectFit: 'contain', borderRadius: 28}} />;
      break;
    case 'quote':
      content = <><div style={{fontSize: 150, color: '#fbbf24', lineHeight: 0.8}}>“</div><blockquote style={{fontSize: portrait ? 48 : 58, lineHeight: 1.65, margin: '30px 0'}}>{visual.text}</blockquote><div style={{fontSize: 29, color: '#94a3b8'}}>— {visual.source}</div></>;
      break;
    case 'diagram':
      content = <Diagram code={visual.mermaid} id={scene.id} />;
      break;
    case 'code':
      content = <div style={{width: '100%', background: '#07101c', border: '1px solid #334155', borderRadius: 24, padding: 36, boxSizing: 'border-box'}}><div style={{fontSize: 24, color: '#7dd3fc', marginBottom: 24}}>{visual.language}</div><pre style={{fontSize: portrait ? 28 : 34, lineHeight: 1.5, margin: 0, whiteSpace: 'pre-wrap', overflowWrap: 'anywhere'}}>{visual.code}</pre></div>;
  }
  return <AbsoluteFill style={{opacity, transform: `translateY(${translate}px)`, padding: portrait ? '280px 88px 540px' : '190px 140px 290px'}}><div style={{fontSize: 22, color: '#64748b', marginBottom: 32, letterSpacing: 4}}>{String(index + 1).padStart(2, '0')} / MINDFRAME</div><div style={{display: 'flex', flexDirection: 'column', justifyContent: 'center', alignItems: visual.type === 'image' || visual.type === 'diagram' ? 'center' : 'stretch', flex: 1, minHeight: 0}}>{content}</div></AbsoluteFill>;
};

const Video: React.FC<VideoProps> = ({storyboard, clips}) => {
  const frame = useCurrentFrame();
  const {width, height, durationInFrames} = useVideoConfig();
  const portrait = height > width;
  const active = clips.find((clip) => frame >= clip.start_frame && frame < clip.start_frame + clip.duration_frames);
  return <AbsoluteFill style={{background: 'radial-gradient(ellipse at 85% 12%, #17354b 0%, #101b2d 38%, #0b1220 75%)', color: '#f8fafc', fontFamily: 'Noto Sans CJK SC, PingFang SC, Microsoft YaHei, Arial, sans-serif'}}>
    <div style={{position: 'absolute', top: portrait ? 130 : 64, left: portrait ? 88 : 140, right: 110, display: 'flex', justifyContent: 'space-between', gap: 50, color: '#94a3b8', fontSize: 25}}><span style={{letterSpacing: 3}}>MINDFRAME</span><span style={{overflow: 'hidden', whiteSpace: 'nowrap', textOverflow: 'ellipsis'}}>{storyboard.title}</span></div>
    {storyboard.scenes.map((scene, index) => {
      const parts = clips.filter((clip) => clip.scene_index === index);
      const from = parts[0].start_frame;
      const duration = parts.reduce((sum, clip) => sum + clip.duration_frames, 0);
      return <Sequence key={scene.id} from={from} durationInFrames={duration}><SceneCard scene={scene} duration={duration} index={index} /></Sequence>;
    })}
    {clips.map((clip) => <Sequence key={clip.audio} from={clip.start_frame} durationInFrames={clip.duration_frames}><Audio src={staticFile(clip.audio)} /></Sequence>)}
    {active && <div style={{position: 'absolute', left: portrait ? 88 : 140, right: portrait ? 120 : 140, bottom: portrait ? 250 : 100, background: 'rgba(3,7,18,0.83)', borderLeft: '5px solid #7dd3fc', borderRadius: 12, padding: '24px 34px', fontSize: portrait ? 42 : 43, lineHeight: 1.55, whiteSpace: 'pre-wrap', overflowWrap: 'anywhere'}}>{storyboard.scenes[active.scene_index].narration[active.utterance_index]}</div>}
    <div style={{position: 'absolute', bottom: 0, left: 0, width: `${(frame + 1) / durationInFrames * 100}%`, height: 5, background: '#7dd3fc'}} />
  </AbsoluteFill>;
};

const Root: React.FC = () => {
  const props = getInputProps<VideoProps>();
  if (!props.clips?.length) throw new Error('Use the MindFrame CLI with a prepared timeline.json');
  const last = props.clips[props.clips.length - 1];
  return <Composition id="MindFrame" component={Video} defaultProps={props} width={props.width} height={props.height} fps={props.fps} durationInFrames={last.start_frame + last.duration_frames} />;
};
registerRoot(Root);
