/// 4-bit Register for CPU
#[derive(Debug, Clone, Copy)]
pub struct Register4Bit {
    value: u8, // 4-bit value (0-15)
}

impl Register4Bit {
    /// Create a new register initialized to 0
    pub fn new() -> Self {
        Register4Bit { value: 0 }
    }

    /// Create a register with initial value
    pub fn with_value(value: u8) -> Self {
        Register4Bit {
            value: value & 0x0F, // Ensure 4-bit
        }
    }

    /// Read the register value
    pub fn read(&self) -> u8 {
        self.value
    }

    /// Write a new value to the register (4-bit masked)
    pub fn write(&mut self, value: u8) {
        self.value = value & 0x0F;
    }

    /// Increment the register (with wrap-around at 16)
    pub fn increment(&mut self) {
        self.value = (self.value + 1) & 0x0F;
    }

    /// Decrement the register (with wrap-around at 0)
    pub fn decrement(&mut self) {
        if self.value == 0 {
            self.value = 0xF; // Wrap to 15
        } else {
            self.value -= 1;
        }
    }

    /// Clear the register (set to 0)
    pub fn clear(&mut self) {
        self.value = 0;
    }

    /// Check if register is zero
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
}

impl Default for Register4Bit {
    fn default() -> Self {
        Self::new()
    }
}

/// CPU Register set for 4-bit computer
#[derive(Debug)]
pub struct CPURegisters {
    /// Accumulator - main working register for arithmetic/logic operations
    pub accumulator: Register4Bit,

    /// Program Counter - points to next instruction to execute
    pub program_counter: Register4Bit,

    /// Instruction Register - holds current instruction being executed
    pub instruction_register: Register4Bit,

    /// Status flags
    pub zero_flag: bool,  // Set when last operation resulted in zero
    pub carry_flag: bool, // Set when last operation had carry/overflow
}

impl CPURegisters {
    /// Create new register set with all registers initialized to 0
    pub fn new() -> Self {
        CPURegisters {
            accumulator: Register4Bit::new(),
            program_counter: Register4Bit::new(),
            instruction_register: Register4Bit::new(),
            zero_flag: false,
            carry_flag: false,
        }
    }

    /// Reset all registers and flags to initial state
    pub fn reset(&mut self) {
        self.accumulator.clear();
        self.program_counter.clear();
        self.instruction_register.clear();
        self.zero_flag = false;
        self.carry_flag = false;
    }

    /// Update flags based on ALU result
    pub fn update_flags(&mut self, value: u8, carry: bool) {
        self.zero_flag = (value & 0x0F) == 0;
        self.carry_flag = carry;
    }

    /// Display register state for debugging
    pub fn display(&self) {
        println!("=== CPU Registers ===");
        println!(
            "A:  0x{:X} ({})",
            self.accumulator.read(),
            self.accumulator.read()
        );
        println!(
            "PC: 0x{:X} ({})",
            self.program_counter.read(),
            self.program_counter.read()
        );
        println!(
            "IR: 0x{:X} ({})",
            self.instruction_register.read(),
            self.instruction_register.read()
        );
        println!(
            "Flags: Z={} C={}",
            if self.zero_flag { "1" } else { "0" },
            if self.carry_flag { "1" } else { "0" }
        );
    }
}

impl Default for CPURegisters {
    fn default() -> Self {
        Self::new()
    }
}