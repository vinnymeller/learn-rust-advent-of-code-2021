use std::io;

struct BingoNumbers {
    numbers: Vec<u32>,
    curr_index: usize,
}

impl BingoNumbers {
    fn new(numbers: Vec<u32>) -> BingoNumbers {
        BingoNumbers {
            numbers,
            curr_index: 0,
        }
    }

    fn build() -> BingoNumbers {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let nums = input.split(',');
        let numbers = input
            .trim()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        BingoNumbers::new(numbers)
    }

    fn get_next_number(&mut self) -> Option<u32> {
        if self.curr_index >= self.numbers.len() {
            return None;
        }
        let next_number = self.numbers[self.curr_index];
        self.curr_index += 1;
        Some(next_number)
    }

    fn reset(&mut self) -> () {
        self.curr_index = 0;
    }
}

struct BingoBoard {
    board: Vec<Vec<u32>>,
    filled: Vec<Vec<bool>>,
}

impl BingoBoard {
    fn new(board: Vec<Vec<u32>>) -> BingoBoard {
        BingoBoard { board, filled: vec![vec![false; 5]; 5] }
    }

    fn build() -> Option<BingoBoard> {
        io::stdin().read_line(&mut String::new()); // ignore first line, newline between boards
        let mut board = Vec::new();
        for _ in 0..5 {
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            if input.trim().len() == 0 {
                return None;
            }
            let row = input
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            board.push(row);
        }
        Some(BingoBoard::new(board))
    }

    fn fill_number(&mut self, number: u32) -> () {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == number {
                    self.filled[i][j] = true;
                }
            }
        }
    }

    fn check_for_winner(&mut self) -> bool {
        let mut winner = false;
        // check rows
        for i in 0..5 {
            let mut row_winner = true;
            for j in 0..5 {
                if !self.filled[i][j] {
                    row_winner = false;
                    break;
                }
            }
            if row_winner {
                winner = true;
                break;
            }
        }
        if winner {
            return true;
        }
        // check columns
        for j in 0..5 {
            let mut col_winner = true;
            for i in 0..5 {
                if !self.filled[i][j] {
                    col_winner = false;
                    break;
                }
            }
            if col_winner {
                winner = true;
                break;
            }
        }
        if winner {
            return true;
        }
        return false;
    }

    fn calculate_unmarked_sum(&self) -> u32 {
        let mut unmarked_sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.filled[i][j] {
                    unmarked_sum += self.board[i][j];
                }
            }
        }
        unmarked_sum as u32
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut bingo_numbers, mut bingo_boards) = read_input();
    // find_first_winner(&mut bingo_numbers, &mut bingo_boards);
    find_last_winner(&mut bingo_numbers, &mut bingo_boards);
    Ok(())
}

fn read_input() -> (BingoNumbers, Vec<BingoBoard>) {
    // read first line of input, the bingo numbers
    let bingo_numbers = BingoNumbers::build();

    let mut bingo_boards: Vec<BingoBoard> = vec![];
    while let Some(bingo_board) = BingoBoard::build() {
        bingo_boards.push(bingo_board);
    }

    (bingo_numbers, bingo_boards)
}

fn find_last_winner(bingo_numbers: &mut BingoNumbers, bingo_boards: &mut Vec<BingoBoard>) -> () {
    let mut bingo_boards_count = bingo_boards.len();
    while let Some(number) = bingo_numbers.get_next_number() {
        println!("Currently on number {}", number);
        let mut idx_to_remove = vec![];
        for (i,bingo_board) in &mut bingo_boards.iter_mut().enumerate() {
            bingo_board.fill_number(number);
            if bingo_board.check_for_winner() {
                if bingo_boards_count == 1 {
                    println!("Final winner!");
                    let unmarked_sum = bingo_board.calculate_unmarked_sum();
                    println!("Unmarked sum: {}, number: {}, product: {}", unmarked_sum, number, unmarked_sum * number);
                    return;
                }
                // remove this bingo board from the list
                idx_to_remove.push(i);
            }
        }
        idx_to_remove.reverse();
        for i in idx_to_remove {
            println!("Removing bingo board {} because it won!", i);
            bingo_boards_count -= 1;
            bingo_boards.remove(i);
        }
    }
}

fn find_first_winner(bingo_numbers: &mut BingoNumbers, bingo_boards: &mut Vec<BingoBoard>) -> () {
    while let Some(number) = bingo_numbers.get_next_number() {
        println!("Currently on number {}", number);
        for (i,bingo_board) in &mut bingo_boards.iter_mut().enumerate() {
            bingo_board.fill_number(number);
            if bingo_board.check_for_winner() {
                println!("BINGO! on board {}", i);
                let unmarked_sum = bingo_board.calculate_unmarked_sum();
                let product = unmarked_sum as u16 * number as u16;
                println!("Unmarked sum: {}, current_num: {}, product: {}", unmarked_sum, number, product);
                return;
            }
        }
    }
}
