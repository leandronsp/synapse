use synapse::cpu::{CPU, Instruction};

#[test]
fn test_cpu_simple_addition() {
    let mut cpu = CPU::new();
    
    // Program: Load 5, Add 3, Store result at address 0xE
    let program = vec![
        0x5, 0x5,  // LDI 5 - Load immediate 5 into accumulator
        0x3, 0xD,  // ADD D - Add memory[D] to accumulator (we'll put 3 there)
        0x2, 0xE,  // STORE E - Store result at address E
        0xF,       // HALT
    ];
    
    // Put data value 3 at address 0xD
    cpu.memory.write(0xD, 3);
    
    // Load and run program
    cpu.load_program(&program);
    cpu.run();
    
    // Check result
    assert_eq!(cpu.memory.read(0xE), 8);
    assert_eq!(cpu.registers.accumulator.read(), 8);
    assert!(cpu.halted);
}

#[test]
fn test_cpu_subtraction() {
    let mut cpu = CPU::new();
    
    // Program: Load 10, Subtract 3, Store result
    let program = vec![
        0x5, 0xA,  // LDI 10 - Load immediate 10
        0x4, 0xD,  // SUB D - Subtract memory[D] from accumulator
        0x2, 0xE,  // STORE E - Store result
        0xF,       // HALT
    ];
    
    cpu.memory.write(0xD, 3);
    cpu.load_program(&program);
    cpu.run();
    
    assert_eq!(cpu.memory.read(0xE), 7);
    assert_eq!(cpu.registers.accumulator.read(), 7);
}

#[test]
fn test_cpu_load_store() {
    let mut cpu = CPU::new();
    
    // Program: Load from memory, store to different location
    let program = vec![
        0x1, 0xC,  // LOAD C - Load memory[C] into accumulator
        0x2, 0xD,  // STORE D - Store accumulator to memory[D]
        0xF,       // HALT
    ];
    
    cpu.memory.write(0xC, 9);
    cpu.load_program(&program);
    cpu.run();
    
    assert_eq!(cpu.memory.read(0xD), 9);
    assert_eq!(cpu.registers.accumulator.read(), 9);
}

#[test]
fn test_cpu_overflow() {
    let mut cpu = CPU::new();
    
    // Program: Test 4-bit overflow (15 + 1 = 0 with carry)
    let program = vec![
        0x5, 0xF,  // LDI 15
        0x3, 0xD,  // ADD D (contains 1)
        0x2, 0xE,  // STORE E
        0xF,       // HALT
    ];
    
    cpu.memory.write(0xD, 1);
    cpu.load_program(&program);
    cpu.run();
    
    assert_eq!(cpu.memory.read(0xE), 0);
    assert!(cpu.registers.carry_flag);
    assert!(cpu.registers.zero_flag);
}

#[test]
fn test_cpu_program_counter() {
    let mut cpu = CPU::new();
    
    // Simple program to test PC advancement
    let program = vec![
        0x0,       // NOP
        0x0,       // NOP
        0x5, 0x7,  // LDI 7
        0xF,       // HALT
    ];
    
    cpu.load_program(&program);
    
    // Step through execution
    cpu.cycle(); // NOP at PC=0
    assert_eq!(cpu.registers.program_counter.read(), 1);
    
    cpu.cycle(); // NOP at PC=1
    assert_eq!(cpu.registers.program_counter.read(), 2);
    
    cpu.cycle(); // LDI at PC=2,3
    assert_eq!(cpu.registers.program_counter.read(), 4);
    assert_eq!(cpu.registers.accumulator.read(), 7);
    
    cpu.cycle(); // HALT at PC=4
    assert!(cpu.halted);
}