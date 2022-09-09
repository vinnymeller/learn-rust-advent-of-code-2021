
use std::io::BufRead;

fn main() {
    let filename = std::env::args().nth(1).expect("No filename given");
    let file = std::fs::File::open(filename).expect("Could not open file");
    let buf = std::io::BufReader::new(file);
    let lines: Vec<_> = buf.lines().map(|l| l.unwrap()).collect();
    let line_len = lines[0].len();
    let mut pos_sum: Vec<i32> = vec![0; line_len];
    for line in &lines[..] {
        for (i, c) in line.chars().enumerate() {
            let c = c.to_digit(10).unwrap();
            if c == 1 {
                pos_sum[i] += 1;
            }
        
        }
    }
    let line_count = lines.len();

    let mut gamma_rate: Vec<i32> = vec![];
    for i in 0..line_len {
        let avg = pos_sum[i] as f32 / line_count as f32;
        if avg >= 0.5 {
            // do something
            gamma_rate.push(1);
        } else {
            gamma_rate.push(0);
        }
    }
    let mut gamma = String::new();
    for bin in gamma_rate {
        gamma.push_str(&format!("{:b}", bin));
    }
    println!("{}", gamma);
    let epsilon = gamma.chars().map(|c| {
        if c == '1' {
            '0'
        } else {
            '1'
        }
    }).collect::<String>();
    println!("{}", epsilon);

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("Gamma * Epsilon: {}", gamma * epsilon);

}
