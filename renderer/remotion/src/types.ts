export type Visual =
  | {type: 'title'; text: string}
  | {type: 'key_point'; title: string; body: string}
  | {type: 'image'; prompt: string}
  | {type: 'quote'; text: string; source: string}
  | {type: 'diagram'; mermaid: string}
  | {type: 'code'; language: string; code: string};

export type Scene = {
  id: string;
  narration: string[];
  point_refs: number[];
  visual: Visual;
  transition: 'cut' | 'fade' | 'slide';
};
export type Clip = {
  scene_index: number; utterance_index: number;
  start_frame: number; duration_frames: number; audio: string;
};
export type VideoProps = {
  schema_version: number; fps: number; width: number; height: number;
  storyboard: {title: string; summary: string; scenes: Scene[]};
  clips: Clip[];
};
