use synapse::cpu::{CPU, Instruction};

#[test]
fn test_cpu_simple_addition() {
    let mut cpu = CPU::new();

    // Program: Load 5, Add 3, Store result at address 0xE
    let program = vec![
        0x5, 0x5, // LDI 5 - Load immediate 5 into accumulator
        0x3, 0xD, // ADD D - Add memory[D] to accumulator (we'll put 3 there)
        0x2, 0xE, // STORE E - Store result at address E
        0xF, // HALT
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
        0x5, 0xA, // LDI 10 - Load immediate 10
        0x4, 0xD, // SUB D - Subtract memory[D] from accumulator
        0x2, 0xE, // STORE E - Store result
        0xF, // HALT
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
        0x1, 0xC, // LOAD C - Load memory[C] into accumulator
        0x2, 0xD, // STORE D - Store accumulator to memory[D]
        0xF, // HALT
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

    // Program: Calculate 15 + 2 = 17, store carry and result separately
    let program = vec![
        0x5, 0xF, // LDI 15
        0x3, 0xD, // ADD D (contains 2)
        0x2, 0xE, // STORE E - Store low nibble (result & 0x0F)
        0xF, // HALT
    ];

    cpu.memory.write(0xD, 2);
    cpu.load_program(&program);
    cpu.run();

    // Result should be 1 (17 & 0x0F = 1) with carry flag set
    let low_nibble = cpu.memory.read(0xE);
    let carry = if cpu.registers.carry_flag { 1 } else { 0 };
    let full_result = (carry * 16) + low_nibble;

    assert_eq!(low_nibble, 1);
    assert!(cpu.registers.carry_flag);
    assert_eq!(full_result, 17);
}

#[test]
fn test_cpu_store_large_number() {
    let mut cpu = CPU::new();

    // Program: Store the number 17 using two 4-bit memory cells
    // 17 = 16 + 1 = 0x10 + 0x01 (high nibble = 1, low nibble = 1)
    let program = vec![
        0x5, 0x1, // LDI 1 - Load high nibble (17 >> 4 = 1)
        0x2, 0xE, // STORE E - Store high nibble at address E
        0x5, 0x1, // LDI 1 - Load low nibble (17 & 0x0F = 1)
        0x2, 0xF, // STORE F - Store low nibble at address F
        0xF, // HALT
    ];

    cpu.load_program(&program);
    cpu.run();

    // Verify that we can reconstruct 17 from the two memory cells
    let high_nibble = cpu.memory.read(0xE);
    let low_nibble = cpu.memory.read(0xF);
    let reconstructed_value = (high_nibble << 4) | low_nibble;

    assert_eq!(high_nibble, 1);
    assert_eq!(low_nibble, 1);
    assert_eq!(reconstructed_value, 17);
}

#[test]
fn test_cpu_program_counter() {
    let mut cpu = CPU::new();

    // Simple program to test PC advancement
    let program = vec![
        0x0, // NOP
        0x0, // NOP
        0x5, 0x7, // LDI 7
        0xF, // HALT
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

