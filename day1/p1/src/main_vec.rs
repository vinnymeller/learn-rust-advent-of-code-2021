use std::io::{BufRead,BufReader};

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let buf = BufReader::new(file);
    let lines: Vec<_> = buf
                        .lines()
                        .map(|x| x.unwrap())
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect();

    let mut count = 0;
    for i in 1..lines.len() {
        if lines[i-1] < lines[i] {
            count += 1;
        }
    }
    println!("{:?}", count);
}
