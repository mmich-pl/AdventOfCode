use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::vec::Vec;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


pub fn main(){
    let lines = lines_from_file("src/data/task1.txt");
    let mut sum = 0;
    let mut sums = Vec::new();
    for line in lines {
        if  line.is_empty() {
                sums.push(sum);
                sum = 0;
        } else { 
            sum += line.parse::<i32>().unwrap();
        }
    }
    
    sums.sort_by(|a, b| b.cmp(a));

    println!("part 1: {:?}", sums.iter().max().unwrap());
    println!("part 2: {:?}", sums[..3].iter().sum::<i32>());   
}