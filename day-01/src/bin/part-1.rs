use std::fs::File;
use std::io::{BufRead, BufReader};

/// Returns number made from first and last digits encountered
fn parse_string(s: String) -> i64 {
    // First and second digit
    let mut d1: char = '0';
    let mut d2: char = '0';

    // Final String
    let mut num = Vec::new();

    // Counter for digits
    let mut counter: i32 = 0;

    let iter = s.chars(); // This returns an iterator of characters

    for c in iter {
        if c.is_digit(10) {
            counter = counter + 1;

            if counter == 1 {d1 = c; d2 = c;} // First digit found (I also set the second so that if only one is found my number will be made from the first digit repeated
            else {d2 = c} // Last digit
        }
    }
    num.push(d1);
    num.push(d2);

    num.into_iter().collect::<String>().parse::<i64>().unwrap()


}

fn main() -> std::io::Result<()>{
    // Opening file and buffer
    let file = File::open("src/bin/input-1.txt")?;
    let reader = BufReader::new(file);

    // Final sum
    let mut sum: i64 = 0;

    // Read the file line by line
    for line in reader.lines() {
        println!("Partial Sum: {} ", sum);
        sum = sum + parse_string(line.unwrap());
    }

    println!("\nFinal sum: {} ", sum);

    Ok(())
}

#[cfg(test)]
#[test]
fn test_parse_string() {
    let s = String::from("treb7uchet");
    let num: i64 = parse_string(s);
    assert_eq!(77, num);
}
