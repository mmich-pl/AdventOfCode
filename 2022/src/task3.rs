use std::fs;

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




pub fn main() {
    let input = fs::read_to_string("src/data/task3.txt").unwrap();
    println!("{}", part_1(&input));
}