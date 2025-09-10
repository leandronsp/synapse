use crate::gates::{GateType, LogicGate};
use crate::half_adder::HalfAdder;

/// Result of full adder computation
#[derive(Debug, PartialEq)]
pub struct FullAdderResult {
    pub sum: bool,
    pub carry: bool,
}

/// A full adder built from two half adders and an OR gate
/// Full Adder = HalfAdder1(A, B) + HalfAdder2(Sum1, Cin) + OR(Carry1, Carry2)
#[derive(Debug)]
pub struct FullAdder {
    half_adder1: HalfAdder, // A + B
    half_adder2: HalfAdder, // (A XOR B) + Cin
    or_gate: LogicGate,     // Carry1 OR Carry2
}

impl FullAdder {
    /// Create a new full adder using two half adders and an OR gate
    pub fn new() -> Self {
        let half_adder1 = HalfAdder::new();
        let half_adder2 = HalfAdder::new();
        let or_gate = LogicGate::new(GateType::OR);

        FullAdder {
            half_adder1,
            half_adder2,
            or_gate,
        }
    }

    /// Compute full adder output for three binary inputs (A, B, Carry_in)
    pub fn compute(&self, a: bool, b: bool, carry_in: bool) -> FullAdderResult {
        // First half adder: A + B
        let result1 = self.half_adder1.compute(a, b);

        // Second half adder: (A XOR B) + Cin
        let result2 = self.half_adder2.compute(result1.sum, carry_in);

        // Final carry: Carry1 OR Carry2
        let carry1_f = if result1.carry { 1.0 } else { 0.0 };
        let carry2_f = if result2.carry { 1.0 } else { 0.0 };
        let final_carry_output = self.or_gate.compute(vec![carry1_f, carry2_f]);

        FullAdderResult {
            sum: result2.sum,
            carry: final_carry_output > 0.5,
        }
    }
}

impl Default for FullAdder {
    fn default() -> Self {
        Self::new()
    }
}