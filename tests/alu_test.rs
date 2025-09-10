use synapse::alu::{ALU, ALUOperation, ALUResult};

#[test]
fn test_alu_addition() {
    let alu = ALU::new();

    // Test basic addition
    assert_eq!(
        alu.compute(5, 3, ALUOperation::Add),
        ALUResult {
            result: 8,
            carry: false,
            zero: false
        }
    );

    assert_eq!(
        alu.compute(14, 1, ALUOperation::Add),
        ALUResult {
            result: 15,
            carry: false,
            zero: false
        }
    );

    // Test addition with overflow
    assert_eq!(
        alu.compute(15, 1, ALUOperation::Add),
        ALUResult {
            result: 0,
            carry: true,
            zero: true
        }
    );

    // Test zero result
    assert_eq!(
        alu.compute(0, 0, ALUOperation::Add),
        ALUResult {
            result: 0,
            carry: false,
            zero: true
        }
    );
}

#[test]
fn test_alu_subtraction() {
    let alu = ALU::new();

    // Test basic subtraction
    assert_eq!(
        alu.compute(8, 3, ALUOperation::Subtract),
        ALUResult {
            result: 5,
            carry: true,
            zero: false
        }
    );

    // Test subtraction resulting in zero
    assert_eq!(
        alu.compute(5, 5, ALUOperation::Subtract),
        ALUResult {
            result: 0,
            carry: true,
            zero: true
        }
    );

    // Test underflow (wraps around in 4-bit)
    assert_eq!(
        alu.compute(3, 5, ALUOperation::Subtract),
        ALUResult {
            result: 14,
            carry: false,
            zero: false
        } // -2 = 14 in 4-bit two's complement
    );
}

#[test]
fn test_alu_4bit_masking() {
    let alu = ALU::new();

    // Test that inputs are masked to 4-bit
    assert_eq!(
        alu.compute(255, 1, ALUOperation::Add), // 255 masked to 15, 15 + 1 = 0 with carry
        ALUResult {
            result: 0,
            carry: true,
            zero: true
        }
    );

    assert_eq!(
        alu.compute(16, 17, ALUOperation::Add), // Both masked to 0 and 1, 0 + 1 = 1
        ALUResult {
            result: 1,
            carry: false,
            zero: false
        }
    );
}

