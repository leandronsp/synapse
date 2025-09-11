# Synapse - Neural CPU

Neural CPU implementation using trained neural networks as logic components.

## Overview

Synapse implements a neural-based computer processor where logic operations are performed by trained neural networks instead of traditional digital circuits. Built on top of the [Aspirina](https://crates.io/crates/aspirina) neural network library.

## Implementation

### Neural Logic Gates
- **AND Gate**: Neural network trained for conjunction logic
- **OR Gate**: Neural network trained for disjunction logic  
- **XOR Gate**: Neural network trained for exclusive OR logic (non-linear separation)

### Neural Arithmetic Components
- **Half Adder**: Combines XOR and AND gates to perform single-bit addition
  - Sum = A XOR B (difference detection)
  - Carry = A AND B (overflow detection)
- **Full Adder**: Uses two half adders and OR gate for three-input addition
  - Handles A + B + Carry_in for multi-bit arithmetic
  - Combines carry outputs: Carry1 OR Carry2
- **4-bit ALU**: Arithmetic unit using neural components
  - Operations: Add, Subtract (two's complement)
  - Status flags: Carry and Zero
  - Ripple carry adder chain for arithmetic

### Memory & Registers
- **Memory**: 16 x 4-bit memory bank for program and data storage
- **Registers**: CPU register set including accumulator, program counter, and instruction register
- **4-bit addressing**: Full 16-word addressable memory space

### Complete Neural CPU
- **Instruction Set**: NOP, LOAD, STORE, ADD, SUB, LDI, HALT
- **Fetch-Decode-Execute**: Standard CPU cycle using neural ALU for arithmetic
- **Program Execution**: Can run simple arithmetic programs
- **Overflow Handling**: Supports larger numbers using multiple memory cells

All neural components are automatically trained when instantiated and perform computations using trained neural networks.

### Usage

```rust
use synapse::cpu::CPU;

// Complete Neural CPU - run the demo
cargo run

// Individual operations
let mut cpu = CPU::new();

// Program: Add 5 + 3 and store result
let program = vec![
    0x5, 0x5,  // LDI 5 - Load immediate 5 into accumulator
    0x3, 0xD,  // ADD D - Add memory[D] to accumulator
    0x2, 0xE,  // STORE E - Store result at address E
    0xF,       // HALT
];

cpu.memory.write(0xD, 3);  // Put value 3 at address D
cpu.load_program(&program);
cpu.run();

assert_eq!(cpu.memory.read(0xE), 8);  // Result: 5 + 3 = 8
```

### Demo Output

Run `cargo run` to see a comprehensive demonstration:

```
=== Synapse Neural CPU - Operations Demo ===

+------------------+-----------+-----------+-----------+-----------------+
|    Operation     |     A     |     B     |  Result   |      Notes      |
+------------------+-----------+-----------+-----------+-----------------+
| ADD   5 +  3     |     5     |     3     |     8     | Basic addition  |
| ADD   7 +  8     |     7     |     8     |    15     | Within 4-bit range |
| ADD  15 +  1     |    15     |     1     |    16     | 4-bit overflow  |
| ADD  12 +  7     |    12     |     7     |    19     | 19 > 15 overflow |
+------------------+-----------+-----------+-----------+-----------------+
| SUB  10 -  3     |    10     |     3     |     7     | Basic subtraction |
| SUB  15 -  9     |    15     |     9     |     6     | Larger subtraction |
| SUB   5 -  8     |     5     |     8     |    13     | Underflow (wrap) |
+------------------+-----------+-----------+-----------+-----------------+
|  25 +  17        |   1, 9    |   1, 1    |    42     | 25 + 17 = 42    |
|  50 +  25        |   3, 2    |   1, 9    |    75     | 50 + 25 = 75    |
| 100 +  55        |   6, 4    |   3, 7    |   155     | 100 + 55 = 155  |
| 200 -  50        |  12, 8    |   3, 2    |   150     | 200 - 50 = 150  |
| 128 -  64        |   8, 0    |   4, 0    |    64     | 128 - 64 = 64   |
+------------------+-----------+-----------+-----------+-----------------+

✓ Neural CPU operations completed!
  All arithmetic performed using trained neural networks!
```

### Testing

```bash
cargo test
```

All neural gates are tested against their expected truth tables.

## Architecture

```
src/
├── gates.rs          # Neural logic gate implementations
├── half_adder.rs     # Neural half adder implementation  
├── full_adder.rs     # Neural full adder implementation
├── alu.rs            # 4-bit Neural ALU implementation
├── memory.rs         # 16 x 4-bit memory implementation
├── registers.rs      # CPU registers and flags
├── cpu.rs            # Complete 4-bit Neural CPU
├── lib.rs            # Module exports
└── main.rs           # XOR training demonstration

tests/
├── gates_test.rs     # Neural gate functionality tests
├── half_adder_test.rs # Neural half adder tests
├── full_adder_test.rs # Neural full adder tests
├── alu_test.rs       # 4-bit ALU tests
├── memory_test.rs    # Memory system tests
├── registers_test.rs # CPU register tests
└── cpu_test.rs       # Complete CPU execution tests

docs/
└── XOR.md           # XOR neural network architecture explanation
```

## Instruction Set

| Opcode | Instruction | Description |
|--------|-------------|-------------|
| 0x0    | NOP         | No operation |
| 0x1    | LOAD addr   | Load memory[addr] into accumulator |
| 0x2    | STORE addr  | Store accumulator into memory[addr] |
| 0x3    | ADD addr    | Add memory[addr] to accumulator |
| 0x4    | SUB addr    | Subtract memory[addr] from accumulator |
| 0x5    | LDI value   | Load immediate value into accumulator |
| 0xF    | HALT        | Stop execution |

## Dependencies

- `aspirina = "0.1.0"` - Neural network training and inference

## Documentation

- `docs/XOR.md` - Detailed explanation of XOR neural network architecture and non-linear separation problem
