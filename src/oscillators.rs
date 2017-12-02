extern crate pcm_flow;

use self::pcm_flow::processor::Processor;
use self::pcm_flow::graph::FrameSet;
use std::f32::consts::PI;

pub struct Sine {
    phase: f32,
    samplerate: usize,
    table: Vec<f32>
}

impl Processor<[f32; 1]> for Sine {

    fn frame_process(&mut self, inputs: &FrameSet<[f32; 1]>, outputs: &mut FrameSet<[f32; 1]>) {
        outputs[0][0] = self.table[(self.phase * self.table.len() as f32) as usize];
        self.phase = inputs[0][0] / self.samplerate as f32;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }

    }

    fn set_samplerate(&mut self, samplerate: usize) {
        self.samplerate = samplerate;
    }

    fn inputs_amt(&self) -> usize {
        1
    }

    fn outputs_amt(&self) -> usize {
        1
    }
}

impl Sine {
    pub fn new(tablesize: usize) -> Self {
        let mut table = Vec::new();
        for i in 0..tablesize {
            let val = f32::sin((i as f32/tablesize as f32) * PI * 2.0);
            table.push(val);
        }
        Sine {
            phase: 0.0,
            samplerate: 1,
            table: table
        }
    }
}

pub struct Saw {
    phase: f32,
    samplerate: usize,
}

impl Processor<[f32; 1]> for Saw {

    fn frame_process(&mut self, 
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        outputs[0][0] = self.phase * 2.0 - 1.0;
        self.phase = inputs[0][0] / self.samplerate as f32;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }

    }

    fn set_samplerate(&mut self, samplerate: usize) {
        self.samplerate = samplerate;
    }

    fn inputs_amt(&self) -> usize {
        1
    }

    fn outputs_amt(&self) -> usize {
        1
    }
}

impl Saw {
    pub fn new() -> Self {
        Saw {
            phase: 0.0,
            samplerate: 1
        }
    }
}

pub struct Tri {
    phase: f32,
    samplerate: usize,
}

impl Processor<[f32; 1]> for Tri {

    fn frame_process(&mut self, 
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        outputs[0][0] = if self.phase < 0.5 
                            { self.phase * 4.0 - 1.0 }
                            else
                            { self.phase * (-4.0) - 1.0 };
        self.phase = inputs[0][0] / self.samplerate as f32;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }

    }

    fn set_samplerate(&mut self, samplerate: usize) {
        self.samplerate = samplerate;
    }

    fn inputs_amt(&self) -> usize {
        1
    }

    fn outputs_amt(&self) -> usize {
        1
    }
}

impl Tri {
    pub fn new() -> Self {
        Tri {
            phase: 0.0,
            samplerate: 1
        }
    }
}

pub struct Square {
    phase: f32,
    samplerate: usize,
}

impl Processor<[f32; 1]> for Square {

    fn frame_process(&mut self, 
                     inputs: &FrameSet<[f32; 1]>,
                     outputs: &mut FrameSet<[f32; 1]>) {
        outputs[0][0] = if self.phase < 0.5 
                            { 1.0 }
                            else
                            { -1.0 };
        self.phase = inputs[0][0] / self.samplerate as f32;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }

    }

    fn set_samplerate(&mut self, samplerate: usize) {
        self.samplerate = samplerate;
    }

    fn inputs_amt(&self) -> usize {
        1
    }

    fn outputs_amt(&self) -> usize {
        1
    }
}

impl Square {
    pub fn new() -> Self {
        Square {
            phase: 0.0,
            samplerate: 1
        }
    }
}
