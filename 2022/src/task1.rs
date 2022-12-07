use std::vec::Vec;

use crate::utils::lines_from_file;


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