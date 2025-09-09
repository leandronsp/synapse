use aspirina::layer::Layer;
use aspirina::matrix::Matrix;
use aspirina::neural_network::NeuralNetwork;

fn main() {
    println!("=== Synapse Neural CPU - XOR Training ===");
    
    // XOR training data
    let input = Matrix::new(vec![
        vec![0.0, 0.0],  // 0 XOR 0 = 0
        vec![0.0, 1.0],  // 0 XOR 1 = 1  
        vec![1.0, 0.0],  // 1 XOR 0 = 1
        vec![1.0, 1.0],  // 1 XOR 1 = 0
    ]);

    let targets = Matrix::new(vec![vec![0.0, 1.0, 1.0, 0.0]]);

    // Create neural network with layers (like in aspirina examples)
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![0.5, 0.5],
            vec![-0.3, -0.3], 
            vec![0.8, -0.8],
            vec![-0.6, 0.6],
        ])),
        Layer::new(Matrix::new(vec![vec![0.9, -0.7, 1.2, -0.4]])),
    ];
    
    let network = NeuralNetwork::new(layers);
    
    println!("Training XOR gate with neural network...");
    
    // Train for 10,000 epochs
    for epoch in 0..10000 {
        network.train(input.clone(), targets.clone());
        
        if epoch % 2000 == 0 || epoch == 9999 {
            println!("Epoch: {}/10000", epoch + 1);
        }
    }
    
    println!("\n=== Training Complete ===");
    println!("Testing trained XOR gate:");
    
    // Test all combinations
    let test_cases = [
        (vec![0.0, 0.0], "0 XOR 0"),
        (vec![0.0, 1.0], "0 XOR 1"),
        (vec![1.0, 0.0], "1 XOR 0"),
        (vec![1.0, 1.0], "1 XOR 1"),
    ];
    
    for (input_vals, description) in test_cases {
        let result = network.predict(Matrix::new(vec![input_vals.clone()]));
        let output = result.data[0][0];
        let expected = if input_vals[0] != input_vals[1] { 1.0 } else { 0.0 };
        
        println!(
            "{} = {:.4} (expected: {:.1})",
            description, output, expected
        );
        
        // Check if learning was successful
        let success = if expected > 0.5 {
            output > 0.5
        } else {
            output < 0.5
        };
        if success {
            println!("  ✓ Correct!");
        } else {
            println!("  ✗ Needs more training");
        }
    }
    
    println!("\n✓ XOR gate training completed!");
}
