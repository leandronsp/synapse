# Synapse - Neural CPU

Neural CPU implementation using trained neural networks as logic components.

## Overview

Synapse implements a neural-based computer processor where logic operations are performed by trained neural networks instead of traditional digital circuits. Built on top of the [Aspirina](https://crates.io/crates/aspirina) neural network library.

## Current Implementation

### Neural Logic Gates
- **AND Gate**: Neural network trained for conjunction logic
- **OR Gate**: Neural network trained for disjunction logic  
- **XOR Gate**: Neural network trained for exclusive OR logic (non-linear separation)

### Neural Arithmetic Components
- **Half Adder**: Combines XOR and AND gates to perform single-bit addition
  - Sum = A XOR B (difference detection)
  - Carry = A AND B (overflow detection)

All neural components are automatically trained when instantiated and perform computations using trained neural networks.

### Usage

```rust
use synapse::gates::{LogicGate, GateType};
use synapse::half_adder::HalfAdder;

// Neural gates
let and_gate = LogicGate::new(GateType::AND);
let result = and_gate.compute(vec![1.0, 1.0]); // ~1.0

let xor_gate = LogicGate::new(GateType::XOR);
let result = xor_gate.compute(vec![1.0, 0.0]); // ~1.0

// Neural half adder
let half_adder = HalfAdder::new();
let result = half_adder.compute(true, true); // HalfAdderResult { sum: false, carry: true }
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
├── lib.rs            # Module exports
└── main.rs           # XOR training demonstration

tests/
├── gates_test.rs     # Neural gate functionality tests
└── half_adder_test.rs # Neural half adder tests

docs/
└── XOR.md           # XOR neural network architecture explanation
```

## Next Steps

### Arithmetic Components
- Full Adder (2 Half Adders + OR gate)  
- 4-bit ALU (arithmetic and logic operations)

### Memory & Control
- 4-bit registers
- 16-word memory system
- Program counter and instruction register

### Neural CPU
- Complete 4-bit instruction set architecture
- Fetch-decode-execute cycle using neural components
- Assembly language support

### Advanced Features
- Neural instruction execution
- 4-bit program execution
- CPU debugging and tracing tools

## Dependencies

- `aspirina = "0.1.0"` - Neural network training and inference

## Documentation

- `docs/XOR.md` - Detailed explanation of XOR neural network architecture and non-linear separation problem