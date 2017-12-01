#[cfg(test)]
mod tests {

    extern crate pcm_flow;

    use super::super::oscillators::Sine;
    use self::pcm_flow::graph::Graph;
    use self::pcm_flow::processor::Processor;

    #[test]
    fn sine_test() {
        let mut graph = Graph::new(10, 41000);
        let sine_id = graph.add_processor(Box::new(Sine::new(1000)));
        graph.set_input_amt(1);
        graph.set_output_amt(1);
        graph.connect_input(0, (sine_id, 0)).unwrap();
        graph.connect_output(0, (sine_id, 0)).unwrap();
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        for _ in 0..10 {
            inputs.push(vec![[440.0]]);
            outputs.push(vec![[0.0]]);
        }
        for _ in 0..1000 {
            graph.process(&inputs, &mut outputs)
        }
    }

}
