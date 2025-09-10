/// 4-bit Memory Cell that stores a single 4-bit value
#[derive(Debug, Clone, Copy)]
pub struct MemoryCell {
    data: u8, // 4-bit value (0-15)
}

impl MemoryCell {
    /// Create a new memory cell with initial value 0
    pub fn new() -> Self {
        MemoryCell { data: 0 }
    }

    /// Create a memory cell with specific initial value
    pub fn with_value(value: u8) -> Self {
        MemoryCell {
            data: value & 0x0F, // Ensure 4-bit
        }
    }

    /// Read the stored value
    pub fn read(&self) -> u8 {
        self.data
    }

    /// Write a new value (4-bit masked)
    pub fn write(&mut self, value: u8) {
        self.data = value & 0x0F;
    }
}

impl Default for MemoryCell {
    fn default() -> Self {
        Self::new()
    }
}

/// 16 x 4-bit Memory Bank for the neural computer
#[derive(Debug)]
pub struct Memory {
    cells: [MemoryCell; 16],
}

impl Memory {
    /// Create new memory with all cells initialized to 0
    pub fn new() -> Self {
        Memory {
            cells: [MemoryCell::new(); 16],
        }
    }

    /// Read from memory at given address (4-bit address: 0-15)
    pub fn read(&self, address: u8) -> u8 {
        let addr = (address & 0x0F) as usize; // Ensure 4-bit address
        self.cells[addr].read()
    }

    /// Write to memory at given address
    pub fn write(&mut self, address: u8, value: u8) {
        let addr = (address & 0x0F) as usize; // Ensure 4-bit address
        self.cells[addr].write(value);
    }

    /// Load program data into memory starting at address 0
    pub fn load_program(&mut self, program: &[u8]) {
        for (i, &instruction) in program.iter().enumerate() {
            if i >= 16 {
                break; // Memory is only 16 cells
            }
            self.write(i as u8, instruction);
        }
    }

    /// Get a snapshot of all memory contents for debugging
    pub fn dump(&self) -> [u8; 16] {
        let mut dump = [0u8; 16];
        for (i, item) in dump.iter_mut().enumerate() {
            *item = self.cells[i].read();
        }
        dump
    }

    /// Clear all memory (set to 0)
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.write(0);
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

