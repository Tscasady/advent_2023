#Day13
vertical reflection test is easy
`
test 1[0..half rounded down -1] [half rounded down..len -1],
test 2 [1..half rounded down] == [half rounded up..len] 


if all [0..half rounded down -1] [half rounded down..len -1] {
    result.0 =  Some(half - 1)
 
 if [1..half rounded down] == [half rounded up..len] {
    result.1 = Some(half)
 }


fn test_1(line: &str) -> bool {
    // Replace this with your actual test 1 implementation
    // For example, checking if the line contains a specific pattern
    line.contains("pattern1")
}

fn test_2(line: &str) -> bool {
    // Replace this with your actual test 2 implementation
    // For example, checking if the line starts with a specific character
    line.starts_with('A')
}

fn main() {
    let lines = vec!["test line 1", "Another line", "Line starting with A"];

    // Initialize variables to store the results of each test
    let mut test_1_passed = true;
    let mut test_2_passed = true;

    // Iterate over each line and perform the tests
    for line in lines.iter() {
        if !test_1(line) {
            // If any line fails test 1, set the flag to false
            test_1_passed = false;
        }

        if !test_2(line) {
            // If any line fails test 2, set the flag to false
            test_2_passed = false;
        }
    }

    // Determine the final result based on the combination of test results
    let final_result = match (test_1_passed, test_2_passed) {
        (true, false) => Some(1),
        (false, true) => Some(2),
        _ => None, // Some lines passed test 1, some passed test 2, or both failed
    };

    // Print or use the final result
    match final_result {
        Some(result) => println!("Final result: {}", result),
        None => println!("Operation failed!"),
    }
}
