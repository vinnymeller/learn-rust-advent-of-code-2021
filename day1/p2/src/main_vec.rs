use std::io::BufRead;

const WINDOW_SIZE: usize = 3;

fn main() {
    let filename = std::env::args().nth(1).expect("No file given");
    let file = std::fs::File::open(filename).expect("Could not open file");
    let buf = std::io::BufReader::new(file);
    let lines: Vec<_> = buf
                        .lines()
                        .map(|l| l.unwrap())
                        .map(|l| l.parse::<i32>().unwrap())
                        .collect();
    let mut count = 0;
    for i in 0..lines.len()-WINDOW_SIZE {
        let w1 = &lines[i..i+WINDOW_SIZE];
        let w2 = &lines[i+1..i+WINDOW_SIZE+1];
        let w1_sum: i32 = w1.iter().sum();
        let w2_sum: i32 = w2.iter().sum();
        if w1_sum < w2_sum {
            count += 1;
        }
    }
    println!("{}", count);
}
