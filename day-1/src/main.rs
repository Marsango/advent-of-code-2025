use std::fs;

struct Dial {
    starting_point: i32,
    zero_count: i32
}

impl Dial {
    fn rotate(&mut self, mut instruction: i32) {
        if instruction.abs() > 99 {
            instruction = instruction % 100;
        }

        self.starting_point += instruction;
        if self.starting_point < 0 {
            self.starting_point += 100;
        } else if self.starting_point > 99 {
            self.starting_point -= 100;
        }
        println!("Current value: {} | {} | {}", self.starting_point, instruction, self.zero_count);
        if self.starting_point == 0 {
            self.zero_count += 1;
        }
    }
}

fn parse_instruction(instruction: &str) -> i32 {
    let direction = instruction.chars().nth(0).expect("Error while reading the string");
    let signal = if direction == 'R' { 1 } else { -1 };

    let num_str = &instruction[1..];
    let rotation_size: i32 = num_str.trim().parse().expect("Error while reading the line");

    signal * rotation_size
}

fn main() {

    let input = fs::read_to_string("./src/input.txt").expect("Error while reading the file.");

    let mut dial = Dial{
        starting_point: 50,
        zero_count: 0
    };

    for line in input.lines() {
        dial.rotate(parse_instruction(line));
    }

    println!("{}", dial.zero_count)
}
