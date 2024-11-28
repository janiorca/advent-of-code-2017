use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/aoc_1a").unwrap();
    let data = input.as_bytes();

    // Part 1
    let mut prev = data.last().unwrap().clone();
    let mut total: u32 = 0;
    for item in data {
        if *item == prev {
            total += (prev - '0' as u8) as u32;
        }
        prev = *item;
    }
    println!("{}", total);

    // Part 2
    let mut total: u32 = 0;
    for pos in 0..data.len() {
        let other = data[ (pos + data.len()/2)%data.len() ] as u32;
        if other == data[ pos ] as u32 {
            total += other - '0' as u32;
        }
    }
    println!("{}", total);
}
