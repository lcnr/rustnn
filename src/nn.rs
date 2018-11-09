#[derive(Clone, Debug)]
pub struct Node {
    weights: Vec<f32>
}

impl Node {
    fn with_weights(n_weights: usize) -> Self {
        Self {
            weights: std::iter::repeat_with(|| {
                use rand::distributions::{Distribution, Uniform};
                Uniform::new_inclusive(-1.0, 1.0).sample(&mut rand::thread_rng())
            }).take(n_weights).collect()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Layer {
    nodes: Vec<Node>
}

impl Layer {
    fn new(node_count: usize, weight_count: usize) -> Self {
        Self { 
            nodes: std::iter::repeat_with(|| Node::with_weights(weight_count)).take(node_count).collect() 
        }
    }
}

#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CreationError {
    NotEnoughLayers,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunError {
    WrongInputCount,
}

impl Network {
    pub fn is_valid_type(layers: &[usize]) -> Result<(), CreationError> {
        if layers.len() < 2 || layers.contains(&0) {
            Err(CreationError::NotEnoughLayers)
        }
        else {
            Ok(())
        }
    }

    pub fn new(layers: &[usize]) -> Result<Self, CreationError> {
        Self::is_valid_type(layers)?;

        let mut network = Network {
            layers: Vec::with_capacity(layers.len())
        };

        network.layers.push(Layer::new(layers[1], layers[0] + 1));

        for &layer in &layers[2..] {
            let prev_node_count = network.layers.last().unwrap().nodes.len();
            network.layers.push(Layer::new(layer, prev_node_count + 1));
        }

        Ok(network)
    }

    fn calculate_value(values: &[f32], weights: &[f32]) -> f32 {
        debug_assert_eq!(values.len(), weights.len() - 1);

        let mut total = 1.0;
        for i in 0..values.len() {
            total += values[i] * weights[i];
        }

        total += weights.last().unwrap();

        crate::bipolar_sigmoid(total, 10.0)
    }

    pub fn run(&self, input: &[f32]) -> Result<Vec<f32>, RunError> {
        if input.len() != self.layers[0].nodes[0].weights.len() - 1 {
            Err(RunError::WrongInputCount)
        }
        else {
            let mut prev_layer = input.to_vec();
            for layer in &self.layers {
                let mut current_layer = Vec::with_capacity(layer.nodes.len());

                for node in layer.nodes.iter() {
                    current_layer.push(Self::calculate_value(&prev_layer, &node.weights));
                }

                prev_layer = current_layer;
            }

            Ok(prev_layer)
        }
    }

    pub fn breed(father: &Self, mother: &Self, p: f64) -> Self {
        let mut child = father.clone();

        use rand::distributions::{Distribution, Bernoulli};
        let d = Bernoulli::new(p);
        for (c_lay, p_lay) in child.layers.iter_mut().zip(&mother.layers) {
            for (c_node, p_node) in c_lay.nodes.iter_mut().zip(&p_lay.nodes) {
                for (c_weight, &p_weight) in c_node.weights.iter_mut().zip(&p_node.weights) {
                    if d.sample(&mut rand::thread_rng()) {
                        *c_weight = p_weight;
                    }
                }
            }
        }  

        child
    }

    pub fn mutate(&mut self, p: f64) {
        use rand::distributions::{Distribution, Bernoulli, Uniform};

        let d = Bernoulli::new(p);
        for layer in &mut self.layers {
            for node in &mut layer.nodes {
                for weight in &mut node.weights {
                    if d.sample(&mut rand::thread_rng()) {
                        *weight = Uniform::new_inclusive(-1.0, 1.0).sample(&mut rand::thread_rng());
                    }
                }
            }
        }  
    }
}

