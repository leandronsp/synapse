use crate::alu::{ALUOperation, ALU};
use crate::memory::Memory;
use crate::registers::CPURegisters;

/// 4-bit CPU Instructions (simplified set)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    NOP,       // 0x0: No operation
    LOAD(u8),  // 0x1: Load memory[addr] into accumulator
    STORE(u8), // 0x2: Store accumulator into memory[addr]
    ADD(u8),   // 0x3: Add memory[addr] to accumulator
    SUB(u8),   // 0x4: Subtract memory[addr] from accumulator
    LDI(u8),   // 0x5: Load immediate value into accumulator
    HALT,      // 0xF: Stop execution
}

/// Simple 4-bit CPU with neural ALU
pub struct CPU {
    pub registers: CPURegisters,
    pub memory: Memory,
    pub alu: ALU,
    pub halted: bool,
}

impl CPU {
    /// Create a new CPU instance
    pub fn new() -> Self {
        CPU {
            registers: CPURegisters::new(),
            memory: Memory::new(),
            alu: ALU::new(),
            halted: false,
        }
    }

    /// Reset CPU to initial state
    pub fn reset(&mut self) {
        self.registers.reset();
        self.memory.clear();
        self.halted = false;
    }

    /// Load a program into memory
    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.load_program(program);
    }

    /// Fetch instruction from memory at PC
    fn fetch(&mut self) -> u8 {
        let pc = self.registers.program_counter.read();
        let instruction = self.memory.read(pc);
        self.registers.instruction_register.write(instruction);
        self.registers.program_counter.increment();
        instruction
    }

    /// Decode instruction byte into Instruction enum
    fn decode(&mut self, opcode: u8) -> Instruction {
        match opcode {
            0x0 => Instruction::NOP,
            0x1 => {
                // LOAD needs operand from next memory location
                let operand = self.fetch();
                Instruction::LOAD(operand)
            }
            0x2 => {
                // STORE needs operand from next memory location
                let operand = self.fetch();
                Instruction::STORE(operand)
            }
            0x3 => {
                // ADD needs operand from next memory location
                let operand = self.fetch();
                Instruction::ADD(operand)
            }
            0x4 => {
                // SUB needs operand from next memory location
                let operand = self.fetch();
                Instruction::SUB(operand)
            }
            0x5 => {
                // LDI needs operand from next memory location
                let operand = self.fetch();
                Instruction::LDI(operand)
            }
            0xF => Instruction::HALT,
            _ => Instruction::NOP,
        }
    }

    /// Execute the given instruction
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NOP => {
                // Do nothing
            }
            Instruction::LOAD(addr) => {
                let value = self.memory.read(addr);
                self.registers.accumulator.write(value);
                self.registers.update_flags(value, false);
            }
            Instruction::STORE(addr) => {
                let value = self.registers.accumulator.read();
                self.memory.write(addr, value);
            }
            Instruction::ADD(addr) => {
                let a = self.registers.accumulator.read();
                let b = self.memory.read(addr);
                let result = self.alu.compute(a, b, ALUOperation::Add);
                self.registers.accumulator.write(result.result);
                self.registers.update_flags(result.result, result.carry);
            }
            Instruction::SUB(addr) => {
                let a = self.registers.accumulator.read();
                let b = self.memory.read(addr);
                let result = self.alu.compute(a, b, ALUOperation::Subtract);
                self.registers.accumulator.write(result.result);
                self.registers.update_flags(result.result, result.carry);
            }
            Instruction::LDI(value) => {
                self.registers.accumulator.write(value);
                self.registers.update_flags(value, false);
            }
            Instruction::HALT => {
                self.halted = true;
            }
        }
    }

    /// Run one fetch-decode-execute cycle
    pub fn cycle(&mut self) {
        if self.halted {
            return;
        }

        let instruction_byte = self.fetch();
        let instruction = self.decode(instruction_byte);
        self.execute(instruction);
    }

    /// Run until HALT instruction
    pub fn run(&mut self) {
        while !self.halted {
            self.cycle();
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}