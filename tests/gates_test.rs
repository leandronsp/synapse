use synapse::gates::{LogicGate, GateType};

#[test]
fn test_and_gate() {
    let gate = LogicGate::new(GateType::AND);
    
    // Test all AND gate combinations
    assert!(gate.compute(vec![0.0, 0.0]) < 0.5); // 0 AND 0 = 0
    assert!(gate.compute(vec![0.0, 1.0]) < 0.5); // 0 AND 1 = 0  
    assert!(gate.compute(vec![1.0, 0.0]) < 0.5); // 1 AND 0 = 0
    assert!(gate.compute(vec![1.0, 1.0]) > 0.5); // 1 AND 1 = 1
}

#[test]
fn test_or_gate() {
    let gate = LogicGate::new(GateType::OR);
    
    // Test all OR gate combinations
    assert!(gate.compute(vec![0.0, 0.0]) < 0.5); // 0 OR 0 = 0
    assert!(gate.compute(vec![0.0, 1.0]) > 0.5); // 0 OR 1 = 1
    assert!(gate.compute(vec![1.0, 0.0]) > 0.5); // 1 OR 0 = 1
    assert!(gate.compute(vec![1.0, 1.0]) > 0.5); // 1 OR 1 = 1
}

#[test]
fn test_xor_gate() {
    let gate = LogicGate::new(GateType::XOR);
    
    // Test all XOR gate combinations  
    assert!(gate.compute(vec![0.0, 0.0]) < 0.5); // 0 XOR 0 = 0
    assert!(gate.compute(vec![0.0, 1.0]) > 0.5); // 0 XOR 1 = 1
    assert!(gate.compute(vec![1.0, 0.0]) > 0.5); // 1 XOR 0 = 1
    assert!(gate.compute(vec![1.0, 1.0]) < 0.5); // 1 XOR 1 = 0
}