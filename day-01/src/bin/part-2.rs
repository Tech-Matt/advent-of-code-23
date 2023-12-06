use std::fs::File;
use std::io::{BufRead, BufReader};


/// Replace every occurrence of string integers like "one" with a real integer in the string
// fn replace_string(s: String) -> String {
    //let s = s.to_lowercase()
      // .replace("one", "1")
      // .replace("two", "2")
      // .replace("three", "3")
      // .replace("four", "4")
      // .replace("five", "5")
      // .replace("six", "6")
      // .replace("seven", "7")
      // .replace("eight", "8")
      // .replace("nine", "9");

    // s
// }

/// From Christopher Biscardi
fn parse_string(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
        .parse::<u32>()
        .expect("should be a valid number")
}


fn main() -> std::io::Result<()>{
    // Opening file and buffer
    let file = File::open("src/bin/input-2.txt")?;
    let reader = BufReader::new(file);

    // Final sum
    let mut sum: u32 = 0;

    // Read the file line by line
    for line in reader.lines() {
        println!("Partial Sum: {} ", sum);
        sum = sum + parse_string(line.unwrap().as_str());
    }

    println!("\nFinal sum: {} ", sum);

    Ok(())
}

#[cfg(test)]
// #[test]
// fn test_replace_string_1() {
//     let a = String::from("twotwo");
//     let a = replace_string(a);
//     assert_eq!("22", a);
// }

// #[test]
// fn test_replace_string_2() {
//     let a = String::from("123");
//     let a = replace_string(a);
//     assert_eq!("123", a);
// }


#[test]
fn parse_calibration_1() {
    let s = "two1nine";
    let num: u32 = parse_string(s);
    assert_eq!(29, num);
}

#[test]
fn parse_calibration_2() {
    let s = "eightwothree";
    let num: u32 = parse_string(s);
    assert_eq!(83, num);
}

#[test]
fn parse_calibration_3() {
    let s = "abcone2threexyz";
    let num: u32 = parse_string(s);
    assert_eq!(13, num);
}

#[test]
fn parse_calibration_4() {
    let s = "xtwone3four";
    let num: u32 = parse_string(s);
    assert_eq!(24, num);
}

#[test]
fn parse_calibration_5() {
    let s = "4nineeightseven2";
    let num: u32 = parse_string(s);
    assert_eq!(42, num);
}

#[test]
fn parse_calibration_6() {
    let s = "zoneight234";
    let num: u32 = parse_string(s);
    assert_eq!(14, num);
}

#[test]
fn parse_calibration_7() {
    let s = "7pqrstsixteen";
    let num: u32 = parse_string(s);
    assert_eq!(76, num);
}