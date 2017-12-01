extern crate pcm_flow;

use self::pcm_flow::processor::Processor;
use self::pcm_flow::graph::FrameSet;

pub struct Hardclip {}

impl Processor<[f32; 1]> for Hardclip {
    fn frame_process(&mut self, 
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        outputs[0][0] = inputs[0][0].max(-1.0).min(1.0);
    }
    fn outputs_amt(&self) -> usize {
        1
    }

    fn inputs_amt(&self) -> usize {
        1
    }
}
