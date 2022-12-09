use std::fs;
use itertools::Itertools;

fn part_1(input: &str) -> u32{
     input.lines().filter_map(|line| {
        let line = line.as_bytes();
        let (start, end) = line.split_at(line.len() / 2);

        start.iter().find(|item| end.contains(item))
            .map(|item| match item {
                b'a'..=b'z' => (item - b'a') +1,
                _ => (item - b'A') + 26 +1
            } as u32 )
    }).sum()
}

fn part_2(input: &str) -> u32{
    input.lines().map(|line| line.as_bytes())
        .tuples()
        .filter_map(|(sack1, sack2, sack3)| {
        sack1.iter().find(|item| sack2.contains(item)&&sack3.contains(item))
            .map(|item| match item {
                b'a'..=b'z' => (item - b'a') +1,
                _ => (item - b'A') + 26 +1
            } as u32 )
    }).sum()
}


pub fn main() {
    let input = fs::read_to_string("src/data/task3.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}