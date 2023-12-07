use std::{fs::File, io::{BufReader, BufRead}};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
struct Cubes {
    id: u32, 
    red: i32,
    green: i32,
    blue: i32,
}

impl Default for Cubes {
    fn default() -> Self {
        Cubes {
            id: 0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Cubes {

    fn new(id: u32, red: i32, green: i32, blue: i32) -> Cubes {
        Cubes {
            id, 
            red,
            green,
            blue,
        }
    }
}
/// This parse a string with this format: "Game N: 2 red, 3 blue; 1 red, 3 green" and
/// returns a Bidimensional Vec of cubes
fn parse_string(s: String) -> Vec<Vec<Cubes>> {
    let cubes: Vec<Vec<Cubes>> = Vec::new();

    // Let's strip the 'Game' word
    let s = s.replace("Game", "");

    // Let's split the id and the rest of the string by :
    let splitted: Vec<&str> = s.split(':').collect();
    let id = splitted[0].parse::<u32>().unwrap();

    // Now let's take the different cubes games and put them in the vector
    let games: Vec<&str> = splitted[1].split(";").collect();

    /* Now we have strings like " 6 red, 1 blue "
     First of all it would be a good idea to strip the spaces in the first and last
     position (we can use trim)
    */

    for elem in games {
        let elem = elem.trim();

        // let's split again by commas (we'll then have string like 6 red, 1 blue)
        let cube: Vec<&str> = elem.split(',').collect();

        for color in cube {
         // let's split again by space to get value and color
         let values: Vec<&str> = color.split(' ').collect();

         let value = values[0];
        

        }

    }
    cubes
}

fn main() {
    let file = File::open("src/bin/input-1.txt").unwrap();
    let reader = BufReader::new(file);

    let sum_ids: u32 = 0;

    // Read the file line by line
    for line in reader.lines() {
        let cubes = parse_string(line.unwrap());
    }
}
    


