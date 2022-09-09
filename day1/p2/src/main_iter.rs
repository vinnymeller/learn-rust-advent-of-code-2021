use std::io::BufRead;

fn main() {
    let filename = std::env::args().nth(1).expect("No file given");
    let file = std::fs::File::open(filename).expect("Could not open file");
    let buf = std::io::BufReader::new(file);
    let mut lines = buf
                .lines()
                .map(|l| l.unwrap())
                .map(|l| l.parse::<i32>().unwrap());

    let mut count = 0;
    let mut first = lines.next().unwrap();
    let mut second = lines.next().unwrap();
    let mut third = lines.next().unwrap();

    while let Some(fourth) = lines.next() {
        let w1_sum = first + second + third;
        let w2_sum = second + third + fourth;
        if w1_sum < w2_sum {
            count += 1;
        }
        (first, second, third) = (second, third, fourth);
    }

    println!("{}", count);
}
