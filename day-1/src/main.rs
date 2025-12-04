use std::fs;

struct Dial {
    starting_point: i32,
    zero_count: i32
}
// Start pointing to 50, zero_count = 0
// L350
// 350 > 99
// self.zero_count += 3
// self.starting_point = 0

// ways to point to 0;
// first way: instruction + self.starting_point > 99;
// second way: - instruction == self.starting_point;
// third way: -instruction > self.starting_point;
impl Dial {
    fn rotate(&mut self, mut instruction: i32) {
        if instruction.abs() > 99 {
            self.zero_count += instruction.abs() / 100;
            instruction = instruction % 100;
        }

        if instruction == 0 {
            return;
        }

        if self.starting_point == -instruction {
            self.zero_count += 1;
        } else if self.starting_point + instruction < 0 && self.starting_point != 0{
            self.zero_count += 1;
        } else if self.starting_point + instruction > 99 {
            self.zero_count += 1;
        }

        self.starting_point += instruction;

        if self.starting_point < 0 {
            self.starting_point += 100;
        } else if self.starting_point > 99 {
            self.starting_point -= 100;
        }
        println!("{} | {}", self.starting_point, self.zero_count);
    }
}

// Im counting too much points to zero
// Im counting 1 per time that instruction is bigger than 100.
// Example: 300 will add 3 to the counter;

// Im counting 1 per time that starting_point became less than zero

// Im counting 1 per time that starting_point became greater than 99

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
