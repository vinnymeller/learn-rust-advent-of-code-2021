use std::io::BufRead;

enum GeneratorType {
    Oxygen,
    Co2,
}

fn main() {
    let lines = read_cli_input_to_lines();
    let avg_vec = get_avg_vec_from_lines(&lines);
    let gamma = get_gamma_rate_str_from_lines(&avg_vec);
    let epsilon = get_epsilon_str_from_gamma(&gamma);
    // convert to i32
    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
    let oxygen_generator_rating = get_generator_rating(&lines, GeneratorType::Oxygen);;
    let co2_scrubber_rating = get_generator_rating(&lines, GeneratorType::Co2);
    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("Gamma * Epsilon: {}", gamma * epsilon);
    println!("Oxygen Generator Rating: {}", oxygen_generator_rating);
    println!("CO2 Scrubber Rating: {}", co2_scrubber_rating);
    println!("Life support rating: {}", oxygen_generator_rating * co2_scrubber_rating);
}

fn get_generator_rating(lines: &Vec<String>, gen_type: GeneratorType) -> i32 {
    let (primary, secondary) = match gen_type {
        GeneratorType::Oxygen => ('1', '0'),
        GeneratorType::Co2 => ('0', '1'),
    };
    let mut lines = lines.clone();
    let line_len = lines[0].len();
    for i in 0..line_len {
        if lines.len() == 1 {
            break;
        }
        let mut avg = 0.0;
        for line in &lines {
            avg += line.chars().nth(i).unwrap().to_digit(10).unwrap() as f32;
        }
        avg /= lines.len() as f32;
        lines = lines.iter().filter(|line| {
            let c = line.chars().nth(i).unwrap();
            match avg {
                avg if avg >= 0.5 => c == primary,
                _ => c == secondary,
            }
        }).map(|line| line.to_string()).collect();

    }
    if lines.len() != 1 {
        panic!("Lines length is not 1");
    } else {
        i32::from_str_radix(&lines[0], 2).unwrap()
    }
}

fn read_cli_input_to_lines() -> Vec<String> {
    let filename = std::env::args().nth(1).expect("No filename given");
    let file = std::fs::File::open(filename).expect("Could not open file");
    let buf = std::io::BufReader::new(file);
    let lines: Vec<_> = buf.lines().map(|l| l.unwrap()).collect();
    lines
}

fn get_gamma_rate_str_from_lines(avg_vec: &Vec<f32>) -> String {
    let mut gamma = String::new();
    for avg in avg_vec {
        if *avg > 0.5 {
            gamma.push('1');
        } else {
            gamma.push('0');
        }
    }
    gamma
}

fn get_epsilon_str_from_gamma(gamma: &str) -> String {
    let epsilon = gamma
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();
    epsilon
}

fn get_avg_vec_from_lines(lines: &Vec<String>) -> Vec<f32> {
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
    let mut avg_vec: Vec<f32> = vec![];
    for i in 0..line_len {
        let avg = pos_sum[i] as f32 / line_count as f32;
        avg_vec.push(avg);
    }
    avg_vec
}
