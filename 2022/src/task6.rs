 fn part_1(input : &str) -> usize {

    input.as_bytes()
        .windows(4)
        .position(|window| {
            window
                .iter()
                .enumerate()
                .all(|(idx, c)| !window[..idx].contains(c))
        })
        .unwrap()
        + 4
}

 pub fn part_2(input : &str) -> usize {
     input.as_bytes()
         .windows(14)
         .position(|window| {
             window
                 .iter()
                 .enumerate()
                 .all(|(idx, c)| !window[..idx].contains(c))
         })
         .unwrap()
         + 14
 }

 pub fn main() {
     let input = std::fs::read_to_string("src/data/task6.txt").unwrap();

     println!("{:?}", part_1(&input));
     println!("{:?}", part_2(&input));
 }