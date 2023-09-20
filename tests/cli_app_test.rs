use regex::Regex;
use std::process::Command;

#[test]
fn test_find_hash_digests_success() {
    let expected_result = "4163, 95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000
11848, cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000
12843, bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000
13467, 42254207576dd1cfb7d0e4ceb1afded40b5a46c501e738159d8ac10b36039000
20215, 1f463eb31d6fa7f3a7b37a80f9808814fc05bf10f01a3f653bf369d7603c8000
28892, dab12874ecae90c0f05d7d87ed09921b051a586c7321850f6bb5e110bc6e2000
";

    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("hash_zero_endings_finder")
        .arg("--")
        .arg("-N")
        .arg("3")
        .arg("-F")
        .arg("6")
        .output();

    let output = output.unwrap().stdout;

    let output = String::from_utf8_lossy(&output).to_string();
    let output = clear_app_response(output);

    println!("{}", output);

    assert_eq!(expected_result, output);
}

fn clear_app_response(response: String) -> String {
    // Define a regular expression pattern to match escape codes
    let escape_code_pattern = r"\x1b\[[0-9;]+m";

    // Create a regex object
    let re = Regex::new(escape_code_pattern).unwrap();

    // Use regex to replace escape codes with an empty string
    let result = re.replace_all(&response, "");
    result.to_string()
}

#[test]
fn test_wrong_parameters_err() {
    let expected_result = "unexpected argument '-1' found";

    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("hash_zero_endings_finder")
        .arg("--")
        .arg("-N")
        .arg("-1")
        .arg("-F")
        .arg("-6")
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&output.stderr).to_string();

    assert!(output.contains(expected_result));
}

#[test]
fn test_no_parameters_err() {
    let expected_result = "arguments were not provided";

    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("hash_zero_endings_finder")
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&output.stderr).to_string();

    assert!(output.contains(expected_result));
}
