use std::{io::{BufRead, BufReader}, path::Path, 
          fs::File, string::ParseError, str::FromStr, cmp::Ordering};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock, 
    Paper,
    Scissors,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            if self == other {Ordering::Equal}
            else if self.greater() == *other {Ordering::Less}
            else {Ordering::Greater}
        )
    } 
}

impl Move{
    fn move_value(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
           }
    }

    fn greater(&self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn play(our: Move, their: Move) -> (i32, i32) {
        let (o, t) = if our == their {
            (3, 3)
        } else if our < their {
            (0, 6)
        } else {
            (6, 0)
        };
        (o + our.move_value(), t + their.move_value())
    }
    
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => panic!("failed to match {}", s),
        }
    }
}


fn lines_from_file(filename: impl AsRef<Path>) -> std::io::Result<Vec<Vec<Move>>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let lines = buf.lines();
    let mut res = Vec::new();
    for line in lines {
        let line = line?;
        let items = line
            .split(' ')
            .map(|n| Move::from_str(n).unwrap())
            .collect();
        res.push(items);
    }
    Ok(res)
}

pub fn main() {
    let lines = lines_from_file("src/data/task2.txt").unwrap();

    let mut sum = 0;

    for item in lines {
       let opponent = item[0];
       let we = item[1];
        let game_results = Move::play(we, opponent);
       sum += game_results.0;
    }

    println!("{:?}", sum);

}