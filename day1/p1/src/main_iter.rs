use std::io::{BufRead,BufReader};

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let buf = BufReader::new(file);
    let mut lines = buf
                .lines()
                .map(|x| x.unwrap())
                .map(|x| x.parse::<i32>().unwrap());

    let mut count = 0;
    let mut x = lines.next().expect("empty file");
    while let Some(y) = lines.next() {
        if x < y {
            count += 1;
        }
        x = y;
    }
    println!("{:?}", count)
}

