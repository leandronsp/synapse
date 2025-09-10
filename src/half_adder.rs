use crate::gates::{GateType, LogicGate};

/// Result of half adder computation
#[derive(Debug, PartialEq)]
pub struct HalfAdderResult {
    pub sum: bool,
    pub carry: bool,
}

/// A half adder that computes sum and carry for two bits
/// Sum = A XOR B, Carry = A AND B
#[derive(Debug)]
pub struct HalfAdder {
    xor_gate: LogicGate,
    and_gate: LogicGate,
}

impl HalfAdder {
    /// Create a new half adder with trained neural gates
    pub fn new() -> Self {
        let xor_gate = LogicGate::new(GateType::XOR);
        let and_gate = LogicGate::new(GateType::AND);

        HalfAdder { xor_gate, and_gate }
    }

    /// Compute half adder output for two binary inputs
    pub fn compute(&self, a: bool, b: bool) -> HalfAdderResult {
        let a_f = if a { 1.0 } else { 0.0 };
        let b_f = if b { 1.0 } else { 0.0 };

        let sum_output = self.xor_gate.compute(vec![a_f, b_f]);
        let carry_output = self.and_gate.compute(vec![a_f, b_f]);

        HalfAdderResult {
            sum: sum_output > 0.5,
            carry: carry_output > 0.5,
        }
    }
}

impl Default for HalfAdder {
    fn default() -> Self {
        Self::new()
    }
}

