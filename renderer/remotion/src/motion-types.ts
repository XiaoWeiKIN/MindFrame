export type MotionScene = {
  id: string;
  start_frame: number;
  duration_frames: number;
  image: string;
  screen_text: string;
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
