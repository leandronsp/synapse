use synapse::half_adder::{HalfAdder, HalfAdderResult};

#[test]
fn test_half_adder_all_cases() {
    let half_adder = HalfAdder::new();
    
    // Test all half adder combinations
    assert_eq!(
        half_adder.compute(false, false),
        HalfAdderResult { sum: false, carry: false }
    ); // 0 + 0 = 00
    
    assert_eq!(
        half_adder.compute(false, true),
        HalfAdderResult { sum: true, carry: false }
    ); // 0 + 1 = 01
    
    assert_eq!(
        half_adder.compute(true, false),
        HalfAdderResult { sum: true, carry: false }
    ); // 1 + 0 = 01
    
    assert_eq!(
        half_adder.compute(true, true),
        HalfAdderResult { sum: false, carry: true }
    ); // 1 + 1 = 10
}

#[test]
fn test_half_adder_sum_xor_logic() {
    let half_adder = HalfAdder::new();
    
    // Sum should be XOR of inputs
    assert_eq!(half_adder.compute(false, false).sum, false ^ false);
    assert_eq!(half_adder.compute(false, true).sum, false ^ true);
    assert_eq!(half_adder.compute(true, false).sum, true ^ false);
    assert_eq!(half_adder.compute(true, true).sum, true ^ true);
}

#[test]
fn test_half_adder_carry_and_logic() {
    let half_adder = HalfAdder::new();
    
    // Carry should be AND of inputs
    assert_eq!(half_adder.compute(false, false).carry, false & false);
    assert_eq!(half_adder.compute(false, true).carry, false & true);
    assert_eq!(half_adder.compute(true, false).carry, true & false);
    assert_eq!(half_adder.compute(true, true).carry, true & true);
}