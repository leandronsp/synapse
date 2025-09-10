use crate::full_adder::FullAdder;

/// 4-bit ALU operations
#[derive(Debug, Clone, PartialEq)]
pub enum ALUOperation {
    Add,      // A + B
    Subtract, // A - B (using two's complement)
}

/// 4-bit ALU result
#[derive(Debug, PartialEq)]
pub struct ALUResult {
    pub result: u8,  // 4-bit result (0-15)
    pub carry: bool, // Carry/overflow flag
    pub zero: bool,  // Zero flag
}

/// 4-bit Arithmetic Logic Unit built from full adders
#[derive(Debug)]
pub struct ALU {
    // Four full adders for arithmetic operations
    adder0: FullAdder, // LSB
    adder1: FullAdder,
    adder2: FullAdder,
    adder3: FullAdder, // MSB
}

impl ALU {
    /// Create a new 4-bit ALU with trained neural components
    pub fn new() -> Self {
        // Create full adders for arithmetic
        // A FullAdder has 2 XOR gates, 2 AND gates, and 1 OR gate
        let adder0 = FullAdder::new();
        let adder1 = FullAdder::new();
        let adder2 = FullAdder::new();
        let adder3 = FullAdder::new();

        ALU {
            adder0,
            adder1,
            adder2,
            adder3,
        }
    }

    /// Perform ALU operation on two 4-bit numbers
    pub fn compute(&self, a: u8, b: u8, operation: ALUOperation) -> ALUResult {
        // Ensure inputs are 4-bit
        // u8 are masked to 4-bit
        // 0001 0100 -> 0000 0100
        // 15 -> 0000 1111 -> &0x0F -> 0000 1111
        let a = a & 0x0F;
        let b = b & 0x0F;

        match operation {
            ALUOperation::Add => self.add(a, b),
            ALUOperation::Subtract => self.subtract(a, b),
        }
    }

    /// 4-bit addition using chain of full adders
    fn add(&self, a: u8, b: u8) -> ALUResult {
        let a_bits = self.to_bits(a); // Convert to array of 4 booleans
        let b_bits = self.to_bits(b);

        // Chain full adders for ripple carry addition
        let result0 = self.adder0.compute(a_bits[0], b_bits[0], false); // A, B, Cin = 0
        let result1 = self.adder1.compute(a_bits[1], b_bits[1], result0.carry); // A, B, Cin = Cout1
        let result2 = self.adder2.compute(a_bits[2], b_bits[2], result1.carry); // A, B, Cin = Cout2
        let result3 = self.adder3.compute(a_bits[3], b_bits[3], result2.carry); // A, B, Cin = Cout3

        let result_bits = [result0.sum, result1.sum, result2.sum, result3.sum];
        let result_value = self.bits_to_u8(result_bits);

        ALUResult {
            result: result_value,
            carry: result3.carry,
            zero: result_value == 0,
        }
    }

    /// 4-bit subtraction using two's complement (A - B = A + (~B + 1))
    fn subtract(&self, a: u8, b: u8) -> ALUResult {
        // Two's complement: invert bits and add 1
        let b_inverted = (!b) & 0x0F;
        let a_bits = self.to_bits(a);
        let b_inv_bits = self.to_bits(b_inverted);

        // Add with initial carry = 1 (for +1 in two's complement)
        let result0 = self.adder0.compute(a_bits[0], b_inv_bits[0], true);
        let result1 = self.adder1.compute(a_bits[1], b_inv_bits[1], result0.carry);
        let result2 = self.adder2.compute(a_bits[2], b_inv_bits[2], result1.carry);
        let result3 = self.adder3.compute(a_bits[3], b_inv_bits[3], result2.carry);

        let result_bits = [result0.sum, result1.sum, result2.sum, result3.sum];
        let result_value = self.bits_to_u8(result_bits);

        ALUResult {
            result: result_value,
            carry: result3.carry,
            zero: result_value == 0,
        }
    }

    /// Convert u8 to array of 4 bits (LSB first)
    fn to_bits(&self, value: u8) -> [bool; 4] {
        [
            (value & 0x01) != 0, // position 0
            (value & 0x02) != 0, // position 1
            (value & 0x04) != 0, // position 2
            (value & 0x08) != 0, // position 3
        ]
    }

    /// Convert array of 4 bits to u8 (LSB first)
    fn bits_to_u8(&self, bits: [bool; 4]) -> u8 {
        let mut result = 0u8;
        if bits[0] {
            result |= 0x01;
        }
        if bits[1] {
            result |= 0x02;
        }
        if bits[2] {
            result |= 0x04;
        }
        if bits[3] {
            result |= 0x08;
        }
        result
    }
}

impl Default for ALU {
    fn default() -> Self {
        Self::new()
    }
}
