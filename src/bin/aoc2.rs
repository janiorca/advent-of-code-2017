use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/aoc_2a").unwrap();
    let lines = input.lines();

    // Part 1
    let mut total = 0;
    for line in lines.clone(){
        let numbers = line.split_whitespace();
        let values: Vec<i32> = numbers.map(|n| n.parse::<i32>().unwrap()).collect();
        let min = *values.iter().min().unwrap();
        let max = *values.iter().max().unwrap();
        total += max-min;
    }
    println!("Part1 = {total}");

    total = 0;
    for line in lines{
        let numbers = line.split_whitespace();
        let values: Vec<i32> = numbers.map(|n| n.parse::<i32>().unwrap()).collect();
        for x in values.clone(){
            for y in values.clone(){
                if x == y {
                    continue;
                }
                if x % y == 0 {
                    total += x as i32/y as i32;
                }
            }
        }
    }
    println!("Part2 = {total}");


}