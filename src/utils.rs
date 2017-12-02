extern crate pcm_flow;

use self::pcm_flow::processor::Processor;
use self::pcm_flow::graph::FrameSet;

pub struct Mul {
    inputs: usize
}

impl Processor<[f32; 1]> for Mul {
    fn frame_process(&mut self, 
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        outputs[0][0] = inputs[0][0];
        for i in 1..self.inputs {
            outputs[0][0] *= inputs[i][0]
        }
    }
    fn outputs_amt(&self) -> usize {
        1
    }

    fn inputs_amt(&self) -> usize {
        self.inputs 
    }
}

impl Mul {
    pub fn new(inputs: usize) -> Self {
        Mul {
            inputs: inputs
        }
    }
}

pub struct Invert { }

impl Processor<[f32; 1]> for Invert {
    fn frame_process(&mut self,
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        outputs[0][0] = - inputs[0][0];
    }
    fn inputs_amt(&self) -> usize {
        1
    }

    fn outputs_amt(&self) -> usize {
        1
    }
}

impl Invert {
    pub fn new() -> Self {
        Invert { }
    }
}

pub struct Constant {
    value: f32
}

impl Processor<[f32; 1]> for Constant {
    fn frame_process(&mut self,
                     _: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        outputs[0][0] = self.value;
    }
    fn inputs_amt(&self) -> usize {
        0
    }

    fn outputs_amt(&self) -> usize {
        1
    }
}

impl Constant {
    pub fn new(value: f32) -> Self {
        Constant {
            value: value
        }
    }
}
