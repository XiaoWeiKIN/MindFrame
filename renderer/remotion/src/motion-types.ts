export type MotionSlot = 'top' | 'left' | 'center' | 'right' | 'bottom';

export type MotionElement =
  | {type: 'text'; id: string; text: string; slot: MotionSlot; emphasis: boolean}
  | {type: 'stat'; id: string; label: string; value: string; slot: MotionSlot; emphasis: boolean}
  | {type: 'relation'; id: string; from: string; to: string; label: string | null; slot: MotionSlot}
  | {type: 'matrix'; id: string; title: string; headers: string[]; rows: string[][]; slot: MotionSlot}
  | {type: 'formula'; id: string; text: string; highlight: string | null; note: string | null; slot: MotionSlot};

export type MotionStep = {
  utterance_index: number;
  start_frame: number;
  duration_frames: number;
  elements: MotionElement[];
};

export type MotionScene = {
  id: string;
  start_frame: number;
  duration_frames: number;
  image: string;
  screen_text: string;
  steps?: MotionStep[];
};

export type MotionSubtitle = {
  scene_id: string;
  utterance_index: number;
  start_frame: number;
  end_frame: number;
  text: string;
};

export type MotionProps = {
  schema_version: 1;
  fps: 30;
  width: number;
  height: number;
  duration_frames: number;
  title: string;
  audio: string;
  scenes: MotionScene[];
  subtitles: MotionSubtitle[];
};
