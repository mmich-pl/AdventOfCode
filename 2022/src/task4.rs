use std::str::FromStr;

use crate::utils::lines_from_file;

fn contains(elf_1:& [i8], elf_2: &[i8]) -> bool {
    elf_1[0] <= elf_2[0] && elf_1[1] >= elf_2[1]
}

fn get_ranges(input: &[String], f: fn(& [i8], &[i8])->bool) -> i8 {
    input.iter().map(|line| {
    let ranges = line.split(',')
                                    .map(|x: &str| x.split('-')
                                                    .flat_map(i8::from_str)
                                                    .collect())
                                    .collect::<Vec<Vec<i8>>>();
      (f(&ranges[0], &ranges[1]) ||f(&ranges[1], &ranges[0]) ) as i8 
    }).sum()
}

pub fn main (){

    let data = lines_from_file("src/data/task4.txt");
    println!("{:?}",get_ranges(&data, contains));
}

