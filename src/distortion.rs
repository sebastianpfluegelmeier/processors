extern crate pcm_flow;

use self::pcm_flow::processor::Processor;
use self::pcm_flow::graph::FrameSet;

use std::f32;
use std::f32::consts::PI;

pub struct AnalogClip {}

impl Processor<[f32; 1]> for AnalogClip {
    fn frame_process(&mut self, 
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        let sig = inputs[0][0];
        let param = inputs[1][0];
        outputs[0][0] = (2.0 / (1.0 + f32::exp(-param * sig)) - 1.0) / 
                        (2.0 / (1.0 + f32::exp(-param)) - 1.0)
    }
    fn outputs_amt(&self) -> usize {
        1
    }

    fn inputs_amt(&self) -> usize {
        2
    }
}

pub struct DigitalClip {}

impl Processor<[f32; 1]> for DigitalClip {
    fn frame_process(&mut self, 
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        let sig = inputs[0][0];
        let param = inputs[1][0];
        outputs[0][0] = f32::min(f32::max((param + 1.0) * sig, -1.0), 1.0);
    }
    fn outputs_amt(&self) -> usize {
        1
    }

    fn inputs_amt(&self) -> usize {
        2
    }
}

pub struct HardCurve {}

impl Processor<[f32; 1]> for HardCurve {
    fn frame_process(&mut self, 
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        let signal = inputs[0][0];
        let param = inputs[1][0];
        outputs[0][0] = (signal * (1.0 + f32::abs(param))) / (1.0 + f32::abs(signal*param));
    }
    fn outputs_amt(&self) -> usize {
        1
    }

    fn inputs_amt(&self) -> usize {
        2
    }
}

pub struct SineWave {}

impl Processor<[f32; 1]> for SineWave {
    fn frame_process(&mut self, 
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        let sig = inputs[0][0];
        let param = inputs[1][0];
        outputs[0][0] = f32::sin(PI * sig * param);
    }
    fn outputs_amt(&self) -> usize {
        1
    }

    fn inputs_amt(&self) -> usize {
        2
    }
}

pub struct SawWave {}

impl Processor<[f32; 1]> for SawWave {
    fn frame_process(&mut self, 
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        let sig = inputs[0][0];
        let param = inputs[1][0];
        outputs[0][0] = 2.0 * (param * sig - f32::floor(param * sig)) - 1.0
    }
    fn outputs_amt(&self) -> usize {
        1
    }

    fn inputs_amt(&self) -> usize {
        2
    }
}

pub struct TriangleWave {}

impl Processor<[f32; 1]> for TriangleWave {
    fn frame_process(&mut self, 
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        let sig = inputs[0][0];
        let param = inputs[1][0];
        outputs[0][0] = 2.0 * f32::abs(2.0 * (param * sig - 0.25 - f32::floor(param * sig - 0.25)) - 1.0) - 1.0
    }
    fn outputs_amt(&self) -> usize {
        1
    }

    fn inputs_amt(&self) -> usize {
        2
    }
}

pub struct PulseWave {}

impl Processor<[f32; 1]> for PulseWave {
    fn frame_process(&mut self,
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        let sig = inputs[0][0];
        let param1 = inputs[1][0];
        let param2 = inputs[2][0];
        outputs[0][0] = if f32::sin(PI * sig * param1) > param2 / 100.0 {1.0} else {-1.0}
    }
    fn outputs_amt(&self) -> usize {
        1
    }

    fn inputs_amt(&self) -> usize {
        2
    }
}
