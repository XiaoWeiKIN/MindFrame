import React from 'react';
import {AbsoluteFill, Audio, Composition, Img, Sequence, getInputProps, interpolate, registerRoot, staticFile, useCurrentFrame, useVideoConfig} from 'remotion';
import type {MotionElement, MotionProps, MotionScene, MotionSlot, MotionStep} from './motion-types';

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

const slotStyle = (slot: MotionSlot, portrait: boolean): React.CSSProperties => {
  const base: React.CSSProperties = {position: 'absolute', display: 'flex', flexDirection: 'column', gap: portrait ? 24 : 18, alignItems: 'stretch'};
  switch (slot) {
    case 'top': return {...base, top: portrait ? '15%' : '14%', left: portrait ? '10%' : '18%', right: portrait ? '10%' : '18%'};
    case 'left': return {...base, top: portrait ? '31%' : '29%', left: portrait ? '7%' : '10%', width: portrait ? '39%' : '35%'};
    case 'right': return {...base, top: portrait ? '31%' : '29%', right: portrait ? '7%' : '10%', width: portrait ? '39%' : '35%'};
    case 'bottom': return {...base, bottom: portrait ? '25%' : '20%', left: portrait ? '10%' : '18%', right: portrait ? '10%' : '18%'};
    default: return {...base, top: portrait ? '31%' : '28%', left: portrait ? '14%' : '20%', right: portrait ? '14%' : '20%'};
  }
};

const elementName = (element: MotionElement): string => {
  switch (element.type) {
    case 'text': return element.text;
    case 'stat': return `${element.label} ${element.value}`;
    case 'matrix': return element.title;
    case 'formula': return element.text;
    case 'relation': return element.label ?? '';
  }
};

const FormulaText: React.FC<{text: string; highlight: string | null}> = ({text, highlight}) => {
  if (!highlight) return <>{text}</>;
  const index = text.indexOf(highlight);
  if (index < 0) return <>{text}</>;
  return <>{text.slice(0, index)}<span style={{color: '#E5BE7A'}}>{highlight}</span>{text.slice(index + highlight.length)}</>;
};

const ElementCard: React.FC<{element: MotionElement; lookup: Map<string, MotionElement>; portrait: boolean}> = ({element, lookup, portrait}) => {
  const common: React.CSSProperties = {
    borderRadius: portrait ? 24 : 18,
    border: '1px solid rgba(241,238,231,.18)',
    background: 'rgba(4,13,23,.48)',
    boxShadow: '0 12px 32px rgba(0,0,0,.18)',
    color: '#F1EEE7',
  };
  switch (element.type) {
    case 'text':
      return <div style={{...common, padding: portrait ? '26px 32px' : '20px 28px', textAlign: 'center', fontFamily: '"Noto Serif CJK SC","Songti SC",serif', fontSize: portrait ? 56 : 48, lineHeight: 1.35, whiteSpace: 'pre-wrap', borderColor: element.emphasis ? 'rgba(205,166,106,.80)' : common.border as string, color: element.emphasis ? '#F5D79B' : '#F1EEE7'}}>{element.text}</div>;
    case 'stat':
      return <div style={{...common, padding: portrait ? '24px 28px' : '18px 24px', borderColor: element.emphasis ? 'rgba(205,166,106,.80)' : common.border as string}}>
        <div style={{fontSize: portrait ? 30 : 26, color: 'rgba(241,238,231,.68)', marginBottom: 8}}>{element.label}</div>
        <div style={{fontSize: portrait ? 62 : 54, fontWeight: 750, color: element.emphasis ? '#E5BE7A' : '#F1EEE7'}}>{element.value}</div>
      </div>;
    case 'relation': {
      const from = lookup.get(element.from);
      const to = lookup.get(element.to);
      return <div style={{...common, padding: portrait ? '20px 26px' : '16px 22px', textAlign: 'center', background: 'rgba(4,13,23,.30)'}}>
        <div style={{fontSize: portrait ? 36 : 32, lineHeight: 1.45}}>{from ? elementName(from) : element.from} <span style={{color: '#E5BE7A', padding: '0 14px'}}>→</span> {to ? elementName(to) : element.to}</div>
        {element.label && <div style={{marginTop: 8, fontSize: portrait ? 26 : 23, color: 'rgba(241,238,231,.68)'}}>{element.label}</div>}
      </div>;
    }
    case 'matrix':
      return <div style={{...common, padding: portrait ? 24 : 18}}>
        <div style={{fontSize: portrait ? 36 : 30, fontWeight: 700, textAlign: 'center', marginBottom: 18}}>{element.title}</div>
        <div style={{display: 'grid', gridTemplateColumns: `repeat(${element.headers.length}, minmax(0,1fr))`, borderTop: '1px solid rgba(241,238,231,.22)', borderLeft: '1px solid rgba(241,238,231,.22)'}}>
          {[...element.headers, ...element.rows.flat()].map((cell, index) => <div key={index} style={{padding: portrait ? '14px 10px' : '10px 8px', borderRight: '1px solid rgba(241,238,231,.22)', borderBottom: '1px solid rgba(241,238,231,.22)', textAlign: 'center', fontSize: portrait ? 27 : 24, fontWeight: index < element.headers.length ? 700 : 500, background: index < element.headers.length ? 'rgba(205,166,106,.10)' : 'transparent'}}>{cell}</div>)}
        </div>
      </div>;
    case 'formula':
      return <div style={{...common, padding: portrait ? '28px 30px' : '22px 28px', textAlign: 'center'}}>
        <div style={{fontFamily: '"STIX Two Math","Times New Roman",serif', fontSize: portrait ? 54 : 50, lineHeight: 1.3, overflowWrap: 'anywhere'}}><FormulaText text={element.text} highlight={element.highlight} /></div>
        {element.note && <div style={{marginTop: 16, fontSize: portrait ? 28 : 25, lineHeight: 1.45, color: 'rgba(241,238,231,.72)', whiteSpace: 'pre-wrap'}}>{element.note}</div>}
      </div>;
  }
};

const PrimitiveLayer: React.FC<{step: MotionStep}> = ({step}) => {
  const frame = useCurrentFrame();
  const {width, height} = useVideoConfig();
  const portrait = height > width;
  const enter = interpolate(frame, [0, Math.min(8, Math.max(1, step.duration_frames - 1))], [0, 1], {extrapolateLeft: 'clamp', extrapolateRight: 'clamp'});
  const lookup = new Map(step.elements.map((element) => [element.id, element]));
  const slots: MotionSlot[] = ['top', 'left', 'center', 'right', 'bottom'];
  return <AbsoluteFill style={{opacity: enter, transform: `translateY(${(1 - enter) * 12}px)`}}>
    {slots.map((slot) => {
      const elements = step.elements.filter((element) => element.slot === slot);
      if (!elements.length) return null;
      return <div key={slot} style={slotStyle(slot, portrait)}>
        {elements.map((element) => <ElementCard key={element.id} element={element} lookup={lookup} portrait={portrait} />)}
      </div>;
    })}
  </AbsoluteFill>;
};

const MotionVideo: React.FC<MotionProps> = ({title, audio, scenes, subtitles, duration_frames}) => {
  const {width, height} = useVideoConfig();
  const portrait = height > width;
  return <AbsoluteFill style={{background: '#071521', color: '#F1EEE7', fontFamily: '"Noto Sans CJK SC","PingFang SC","Microsoft YaHei",sans-serif'}}>
    {scenes.map((scene) => <Sequence key={scene.id} from={scene.start_frame} durationInFrames={scene.duration_frames}>
      <Background scene={scene} globalStart={scene.start_frame} total={duration_frames} />
      {scene.steps?.length
        ? scene.steps.map((step) => <Sequence key={`${scene.id}-step-${step.utterance_index}`} from={step.start_frame - scene.start_frame} durationInFrames={step.duration_frames}><PrimitiveLayer step={step} /></Sequence>)
        : <Emphasis text={scene.screen_text} duration={scene.duration_frames} />}
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
