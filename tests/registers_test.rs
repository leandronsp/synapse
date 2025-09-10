use synapse::registers::{Register4Bit, CPURegisters};

#[test]
fn test_register_4bit_operations() {
    let mut reg = Register4Bit::new();
    
    // Test initial value
    assert_eq!(reg.read(), 0);
    assert!(reg.is_zero());
    
    // Test write and read
    reg.write(7);
    assert_eq!(reg.read(), 7);
    assert!(!reg.is_zero());
    
    // Test 4-bit masking
    reg.write(255);
    assert_eq!(reg.read(), 15); // 255 & 0x0F = 15
    
    // Test with_value constructor and masking
    assert_eq!(Register4Bit::with_value(10).read(), 10);
    assert_eq!(Register4Bit::with_value(20).read(), 4); // 20 & 0x0F = 4
}

#[test]
fn test_register_increment_decrement() {
    let mut reg = Register4Bit::new();
    
    // Test increment with wrap-around
    reg.write(15);
    reg.increment();
    assert_eq!(reg.read(), 0);
    
    // Test decrement with wrap-around
    reg.decrement();
    assert_eq!(reg.read(), 15);
}

#[test]
fn test_cpu_registers() {
    let mut regs = CPURegisters::new();
    
    // Test initialization
    assert_eq!(regs.accumulator.read(), 0);
    assert_eq!(regs.program_counter.read(), 0);
    assert_eq!(regs.instruction_register.read(), 0);
    assert!(!regs.zero_flag);
    assert!(!regs.carry_flag);
    
    // Test register operations
    regs.accumulator.write(5);
    regs.program_counter.write(3);
    regs.instruction_register.write(255); // Should mask to 15
    
    assert_eq!(regs.accumulator.read(), 5);
    assert_eq!(regs.program_counter.read(), 3);
    assert_eq!(regs.instruction_register.read(), 15);
    
    // Test flag updates
    regs.update_flags(0, true);
    assert!(regs.zero_flag);
    assert!(regs.carry_flag);
    
    regs.update_flags(7, false);
    assert!(!regs.zero_flag);
    assert!(!regs.carry_flag);
    
    // Test reset
    regs.reset();
    assert_eq!(regs.accumulator.read(), 0);
    assert_eq!(regs.program_counter.read(), 0);
    assert_eq!(regs.instruction_register.read(), 0);
    assert!(!regs.zero_flag);
    assert!(!regs.carry_flag);
}