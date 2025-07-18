fn main() {
    let mut game = UltTickTackToe::new();
    loop {
        if game.three_in_row() {
            game.win();
            return;
        } else {
            game.choice_symbol();
            game.print_board();
            game.place_on_board();
        }
    }
}

struct UltTickTackToe {
    board: Vec<char>,
    symbol: char,
}

impl UltTickTackToe {
    fn input(&self, message: &str) -> char {
        let mut input = String::new();

        loop {
            println!("{}", message);
            input.clear();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().chars().next() {
                Some(c) => {
                    if self.board.contains(&c) {
                        return c;
                    } else {
                        println!(
                            "Invalid input. The character '{}' is already on the board or not allowed.",
                            c
                        );
                    }
                }
                None => {
                    println!("Invalid input. Please enter a single character.");
                }
            }
        }
    }

    fn new() -> Self {
        let mut board: Vec<char> = Vec::new();
        for i in 0..9 {
            board.push(i.to_string().chars().nth(0).unwrap());
        }
        let symbol: char = 'O';

        Self { board, symbol }
    }

    fn choice_symbol(&mut self) {
        if self.symbol == 'X' {
            self.symbol = 'O'
        } else if self.symbol == 'O' {
            self.symbol = 'X'
        } else {
            println!("symbol is wrong {:?}", self.symbol)
        }
    }

    fn print_board(&self) {
        for (index, i) in self.board.iter().enumerate() {
            print!("{} ", i);
            if (index + 1) % 3 == 0 {
                println!();
            }
        }
        println!()
    }
    fn place_on_board(&mut self) {
        let input = self.input("where do you want to place your (write the number it is on)");
        let idx = input
            .to_string()
            .parse::<usize>()
            .expect("Failed to read line");
        self.board[idx] = self.symbol
    }

    fn win(&self) {
        println!("{} has won", self.symbol);
        self.print_board();
    }

    fn three_in_row(&mut self) -> bool {
        //check diagonal
        if self.board[0] == self.symbol
            && self.board[4] == self.symbol
            && self.board[8] == self.symbol
            || self.board[2] == self.symbol
                && self.board[4] == self.symbol
                && self.board[6] == self.symbol
        {
            return true;
        }
        //check loddrett
        for i in 0..3 {
            if self.board[i] == self.symbol
                && self.board[i + 3] == self.symbol
                && self.board[i + 6] == self.symbol
            {
                return true;
            }
        }
        //check vannrett
        for i in 0..3 {
            if self.board[i] == self.symbol
                && self.board[i + 1] == self.symbol
                && self.board[i + 2] == self.symbol
            {
                return true;
            }
        }
        if self.board[0..9].iter().all(|x| *x == 'X' || *x == 'O') {
            self.symbol = 'N';
            return true;
        }

        false
    }
}
