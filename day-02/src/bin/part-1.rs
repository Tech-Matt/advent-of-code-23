use std::{fs::File, io::{BufReader, BufRead, Write}, collections::HashMap};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

// Hashmap for storing id, red, green and blue values (represent a single extraction)
#[derive(Debug)]
struct Cubes {
    id: u32 ,
    map: HashMap<String, u32> ,
}

impl Default for Cubes {
    fn default() -> Self {
        let values = [
            ("red".to_string(), 0),
            ("green".to_string(), 0), 
            ("blue".to_string(), 0)
        ];
        Self { id: 0, map: HashMap::from(values) }
    }
}


/// This parse a string with this format: "Game N: 2 red, 3 blue; 1 red, 3 green" and
/// returns a Bidimensional Vec of cubes
fn parse_string(s: String) -> Vec<Cubes> {
    let mut cubes: Vec<Cubes> = Vec::new();

    // Let's strip the 'Game' word
    let s = s.replace("Game", "");

    // Let's split the id and the rest of the string by :
    let splitted: Vec<&str> = s.split(':').collect();
    // println!("Parsing ID: {}", splitted[0]);
    // std::io::stdout().flush().unwrap();
    let id = splitted[0].trim().parse::<u32>().expect("Parsing ID failed");

    // Now let's take the different cubes games and put them in the vector
    let games: Vec<&str> = splitted[1].split(";").collect();

    /* Now we have strings like " 6 red, 1 blue "
     First of all it would be a good idea to strip the spaces in the first and last
     position (we can use trim)
    */

    for elem in games {

        let mut extraction = Cubes::default();

        let elem = elem.trim();

        // let's split again by commas (we'll then have string like 6 red, 1 blue)
        let cube: Vec<&str> = elem.split(',').collect();

        for pair in cube {
         // let's split again by space to get value and color. We will get the instance of a game. we
         // have to store the values first in the struct and then in the vector
         let values: Vec<&str> = pair.split(' ').collect();

         let value = values[0].parse::<u32>().expect("Error parsing value");
         
        //  match value {
        //     Ok(value) => _,
        //     Err(e) => {println!("Error parsing"); panic!();}
        //  }
         let color = values[1].to_string().to_lowercase();
        
        // Insert elements in the hashmap and id
        extraction.map.insert(color, value);
        extraction.id = id;

        }

        // Add extraction to the vec
        cubes.push(extraction);

    }
    cubes
}

fn main() {
    let file = File::open("src/bin/input-1.txt").expect("File not found");
    let reader = BufReader::new(file);

    let sum_ids: u32 = 0;

    // Read the file line by line
    for line in reader.lines() {
        let cubes = parse_string(line.unwrap());
        for elem in cubes {
            println!("Id: {}, Map: {:?}", elem.id, elem.map);
        }
    }
}


#[cfg(test)]
#[test]
fn test_cubes() {
    let s = String::from(" red 13, green 3; blue 15, red 2 ");
    let c = parse_string(s);

    for elem in c {
        println!("Id: {}, Map: {:?} ", elem.id, elem.map);
    }
    
}
    


