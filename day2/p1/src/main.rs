use std::io::BufRead;

#[derive(Debug)]
enum Movement {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug)]
struct Submarine {
    x: i32,
    y: i32,
}

impl Submarine {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_sub(&mut self, movement: Movement) {
        match movement {
            Movement::Forward(x) => self.x += x,
            Movement::Up(y) => self.y -= y,
            Movement::Down(y) => self.y += y,
        }
    }
}

fn movement_from_str(s: &str) -> Movement {
    let mut splits = s.split_whitespace();
    let command = splits.next().unwrap();
    let value = splits.next().unwrap().parse::<i32>().unwrap();
    match command {
        x if x.starts_with("f") => Movement::Forward(value),
        x if x.starts_with("u") => Movement::Up(value),
        x if x.starts_with("d") => Movement::Down(value),
        _ => panic!("Invalid command"),
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("No input file given");
    let file = std::fs::File::open(filename).expect("Could not open file");
    let buf = std::io::BufReader::new(file);
    let lines = buf
        .lines()
        .map(|l| l.unwrap())
        .map(|l| movement_from_str(&l));


    let mut sub = Submarine::new();
    for movement in lines {
        sub.move_sub(movement);
    }

    println!("x: {}, y: {}, comb: {}", sub.x, sub.y, sub.x * sub.y);
}
