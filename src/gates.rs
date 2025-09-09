use aspirina::layer::Layer;
use aspirina::matrix::Matrix;
use aspirina::neural_network::NeuralNetwork;

/// Represents available logic gates used in the neural CPU
#[derive(Debug, Clone)]
pub enum GateType {
    AND,
    OR,
    XOR,
}

/// A neural logic gate that performs boolean operations using trained neural networks
#[derive(Debug)]
pub struct LogicGate {
    gate_type: GateType,
    network: NeuralNetwork,
}

impl LogicGate {
    /// Creates a new logic gate of the specified type and trains it immediately
    pub fn new(gate_type: GateType) -> Self {
        let network = match gate_type {
            GateType::AND => create_and_network(),
            GateType::OR => create_or_network(),
            GateType::XOR => create_xor_network(),
        };

        let gate = LogicGate { gate_type, network };
        gate.train(10_000); // Train immediately when gate is instantiated
        gate
    }

    /// Train the gate with appropriate training data
    fn train(&self, epochs: usize) {
        let input = Matrix::new(vec![
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
        ]);

        let targets = match self.gate_type {
            GateType::AND => Matrix::new(vec![vec![0.0, 0.0, 0.0, 1.0]]),
            GateType::OR => Matrix::new(vec![vec![0.0, 1.0, 1.0, 1.0]]),
            GateType::XOR => Matrix::new(vec![vec![0.0, 1.0, 1.0, 0.0]]),
        };

        for _ in 0..epochs {
            self.network.train(input.clone(), targets.clone());
        }
    }

    /// Compute the gate output for given inputs
    pub fn compute(&self, inputs: Vec<f64>) -> f64 {
        let input_matrix = Matrix::new(vec![inputs]);
        let result = self.network.predict(input_matrix);
        result.data[0][0]
    }

    /// Get the gate type
    pub fn gate_type(&self) -> &GateType {
        &self.gate_type
    }
}

// Network creation functions for each gate type

fn create_and_network() -> NeuralNetwork {
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![0.8, 0.8],
            vec![0.6, 0.6],
            vec![-0.3, -0.3],
        ])),
        Layer::new(Matrix::new(vec![vec![1.2, 0.8, -0.5]])),
    ];
    NeuralNetwork::new(layers)
}

fn create_or_network() -> NeuralNetwork {
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![1.0, 1.0],
            vec![0.5, 0.5],
            vec![-0.2, -0.2],
        ])),
        Layer::new(Matrix::new(vec![vec![1.5, 1.0, -0.3]])),
    ];
    NeuralNetwork::new(layers)
}

fn create_xor_network() -> NeuralNetwork {
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![0.5, 0.5],
            vec![-0.3, -0.3],
            vec![0.8, -0.8],
            vec![-0.6, 0.6],
        ])),
        Layer::new(Matrix::new(vec![vec![0.9, -0.7, 1.2, -0.4]])),
    ];
    NeuralNetwork::new(layers)
}