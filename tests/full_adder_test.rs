use synapse::full_adder::{FullAdder, FullAdderResult};

#[test]
fn test_full_adder_all_cases() {
    let full_adder = FullAdder::new();

    // Test all 8 possible combinations for full adder
    assert_eq!(
        full_adder.compute(false, false, false),
        FullAdderResult {
            sum: false,
            carry: false
        }
    ); // 0 + 0 + 0 = 00

    assert_eq!(
        full_adder.compute(false, false, true),
        FullAdderResult {
            sum: true,
            carry: false
        }
    ); // 0 + 0 + 1 = 01

    assert_eq!(
        full_adder.compute(false, true, false),
        FullAdderResult {
            sum: true,
            carry: false
        }
    ); // 0 + 1 + 0 = 01

    assert_eq!(
        full_adder.compute(false, true, true),
        FullAdderResult {
            sum: false,
            carry: true
        }
    ); // 0 + 1 + 1 = 10

    assert_eq!(
        full_adder.compute(true, false, false),
        FullAdderResult {
            sum: true,
            carry: false
        }
    ); // 1 + 0 + 0 = 01

    assert_eq!(
        full_adder.compute(true, false, true),
        FullAdderResult {
            sum: false,
            carry: true
        }
    ); // 1 + 0 + 1 = 10

    assert_eq!(
        full_adder.compute(true, true, false),
        FullAdderResult {
            sum: false,
            carry: true
        }
    ); // 1 + 1 + 0 = 10

    assert_eq!(
        full_adder.compute(true, true, true),
        FullAdderResult {
            sum: true,
            carry: true
        }
    ); // 1 + 1 + 1 = 11
}

#[test]
fn test_full_adder_arithmetic() {
    let full_adder = FullAdder::new();

    // Test arithmetic equivalence: result = a + b + carry_in
    let test_cases = [
        (false, false, false, 0), // 0 + 0 + 0 = 0
        (false, false, true, 1),  // 0 + 0 + 1 = 1
        (false, true, false, 1),  // 0 + 1 + 0 = 1
        (false, true, true, 2),   // 0 + 1 + 1 = 2
        (true, false, false, 1),  // 1 + 0 + 0 = 1
        (true, false, true, 2),   // 1 + 0 + 1 = 2
        (true, true, false, 2),   // 1 + 1 + 0 = 2
        (true, true, true, 3),    // 1 + 1 + 1 = 3
    ];

    // true as u8 = 1
    // false as u8 = 0
    for (a, b, carry_in, expected_sum) in test_cases.iter() {
        let result = full_adder.compute(*a, *b, *carry_in);
        let actual_sum = (result.carry as u8) * 2 + (result.sum as u8);
        assert_eq!(actual_sum, *expected_sum);
    }
}

