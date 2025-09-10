use synapse::memory::{Memory, MemoryCell};

#[test]
fn test_memory_cell_operations() {
    let mut cell = MemoryCell::new();
    
    // Test initial value
    assert_eq!(cell.read(), 0);
    
    // Test write and read
    cell.write(7);
    assert_eq!(cell.read(), 7);
    
    // Test 4-bit masking
    cell.write(255);
    assert_eq!(cell.read(), 15); // 255 & 0x0F = 15
    
    // Test with_value constructor
    let cell_with_value = MemoryCell::with_value(10);
    assert_eq!(cell_with_value.read(), 10);
    
    // Test with_value masking
    let cell_masked = MemoryCell::with_value(20);
    assert_eq!(cell_masked.read(), 4); // 20 & 0x0F = 4
}

#[test]
fn test_memory_read_write() {
    let mut memory = Memory::new();
    
    // Test initial state (all zeros)
    for i in 0..16 {
        assert_eq!(memory.read(i), 0);
    }
    
    // Test basic write and read
    memory.write(0, 15);
    memory.write(5, 7);
    memory.write(15, 1);
    
    assert_eq!(memory.read(0), 15);
    assert_eq!(memory.read(5), 7);
    assert_eq!(memory.read(15), 1);
    
    // Test 4-bit value masking
    memory.write(3, 255);
    assert_eq!(memory.read(3), 15); // 255 & 0x0F = 15
    
    // Test 4-bit address masking
    memory.write(16, 9); // Address 16 & 0x0F = 0
    assert_eq!(memory.read(0), 9);
    
    memory.write(21, 3); // Address 21 & 0x0F = 5
    assert_eq!(memory.read(5), 3);
}

#[test]
fn test_memory_load_program() {
    let mut memory = Memory::new();
    
    // Test loading a small program
    let program = vec![1, 2, 3, 4, 5, 6, 7, 8];
    memory.load_program(&program);
    
    for (i, &expected) in program.iter().enumerate() {
        assert_eq!(memory.read(i as u8), expected);
    }
    
    // Test loading program larger than memory
    let large_program = vec![0; 20];
    memory.load_program(&large_program);
    
    // Should only load first 16 values
    for i in 0..16 {
        assert_eq!(memory.read(i), 0);
    }
}

#[test]
fn test_memory_dump() {
    let mut memory = Memory::new();
    
    // Set up some test data
    for i in 0..16u8 {
        memory.write(i, i);
    }
    
    // Test dump
    let dump = memory.dump();
    for i in 0..16 {
        assert_eq!(dump[i], i as u8);
    }
    
    // Test dump with masked values
    memory.write(0, 20); // 20 & 0x0F = 4
    let dump2 = memory.dump();
    assert_eq!(dump2[0], 4);
}

#[test]
fn test_memory_clear() {
    let mut memory = Memory::new();
    
    // Write some data
    for i in 0..16u8 {
        memory.write(i, i + 1);
    }
    
    // Verify data is written
    assert_eq!(memory.read(5), 6);
    assert_eq!(memory.read(10), 11);
    
    // Clear memory
    memory.clear();
    
    // Verify all cells are zero
    for i in 0..16 {
        assert_eq!(memory.read(i), 0);
    }
}

#[test]
fn test_memory_boundaries() {
    let mut memory = Memory::new();
    
    // Test boundary addresses
    memory.write(0, 1);
    memory.write(15, 15);
    
    assert_eq!(memory.read(0), 1);
    assert_eq!(memory.read(15), 15);
    
    // Test wraparound addressing
    memory.write(31, 7); // 31 & 0x0F = 15
    assert_eq!(memory.read(15), 7);
    
    memory.write(32, 8); // 32 & 0x0F = 0
    assert_eq!(memory.read(0), 8);
}