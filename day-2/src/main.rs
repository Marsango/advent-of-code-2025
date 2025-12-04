use std::fs;

fn is_invalid_id(id: i128) -> bool {
    let string = id.to_string();
    if string.len() % 2 != 0 {
        return false
    }
    let half_size = string.len() / 2;
    let first_string = &mut string[0..half_size].chars();
    let second_string = &mut string[half_size..].chars();
    for _ in 0..half_size{
        if first_string.nth(0).unwrap() != second_string.nth(0).unwrap() {
            return false
        }
    }
    true
}

fn process_range(range: &str) -> i128 {
    let mut range_sum: i128 = 0;
    let mut array_range = range.trim().split("-");
    let start = array_range.next().expect("Can't found the start id;");
    let end = array_range.next().expect("Can't found the end id");
    let start: i128 = start.parse().expect("The start id is not a number");
    let end: i128 = end.parse().expect("The start id is not a number");
    for i in start..end + 1 {
        if is_invalid_id(i) {
            range_sum += i;
        }
    }
    range_sum
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Error while reading the input.");
    let mut sum: i128 = 0;
    for range in input.split(","){
        sum += process_range(range);
    }
    println!("{}", sum)
}
