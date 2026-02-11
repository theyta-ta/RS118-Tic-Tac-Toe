use std::io;

#[derive(Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

struct GameState {
    board: [Option<Player>; 9],
    turn: Player,
}

fn print_board(gs: &GameState) {
    println!("BOARD");

    for y in 0..3 {
        let mut to_display = String::new();

        for x in 0..3 {
            let val: Option<Player> = gs.board[(y * 3) + x];

            match val {
                None => {
                    to_display.push('.');
                }
                Some(player) => match player {
                    Player::X => {
                        to_display.push('X');
                    }
                    Player::O => {
                        to_display.push('O');
                    }
                },
            }
        }

        println!("{}", to_display);
    }
}

fn print_turn(gs: &GameState) {
    match gs.turn {
        Player::O => println!("It is O's turn!"),
        Player::X => println!("It is X's turn!"),
    }
}

fn request_int() -> u32 {
    let mut tr: u32 = 0;

    loop {
        let mut inp = String::new();

        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read input");

        match inp.trim().parse() {
            Ok(num) => {
                tr = num;
                break;
            }
            Err(_) => println!("Not an unsigned int! Try again."),
        }
    }

    tr
}

fn request_bound_int() -> u32 {
    let mut tr = 0;

    loop {
        let n = request_int();

        if (n < 4) && (n > 0) {
            tr = n;
            break;
        }

        println!("Put in the range 1-3!");
    }

    tr
}

fn request_guess(gs: &mut GameState) {
    while true {
        println!("Enter x coord:");
        let x = request_bound_int() - 1;

        println!("Enter y coord:");
        let y = request_bound_int() - 1;

        // check if occupied already
        let i: usize = ((y * 3) + x).try_into().unwrap();

        match gs.board[i] {
            Some(_) => {
                println!("Coordinate already occupied!");
            }
            None => {
                gs.board[i] = Some(gs.turn);
                return;
            }
        }
    }
}

fn swap_turn(gs: &mut GameState) {
    match gs.turn {
        Player::O => gs.turn = Player::X,
        Player::X => gs.turn = Player::O,
    }
}

fn check_for_win(gs: &GameState) -> Option<Player> {
    // check for horizontals
    for x in 0..2 {
        if (gs.board[x] == gs.board[3 + x])
            && (gs.board[x] == gs.board[6 + x])
            && gs.board[x].is_some()
        {
            return gs.board[x];
        }
    }

    // check for verticals
    for y in 0..2 {
        if (gs.board[y * 3] == gs.board[1 + y * 3])
            && (gs.board[y * 3] == gs.board[2 + y * 3])
            && gs.board[y * 3].is_some()
        {
            return gs.board[y * 3];
        }
    }

    // check for diagonals
    if (gs.board[0] == gs.board[4]) && (gs.board[4] == gs.board[8]) && gs.board[0].is_some() {
        return gs.board[0];
    }

    if (gs.board[2] == gs.board[4]) && (gs.board[4] == gs.board[6]) && gs.board[2].is_some() {
        return gs.board[0];
    }

    None
}
fn main() {
    let mut our_game = GameState {
        board: [None; 9],
        turn: Player::X,
    };

    let mut won = false;

    while !won {
        print_board(&our_game);
        print_turn(&our_game);
        request_guess(&mut our_game);
        swap_turn(&mut our_game);

        match check_for_win(&our_game) {
            Some(winning_player) => {
                won = true;
                print_board(&our_game);

                match winning_player {
                    Player::O => {
                        println!("Winner: O!");
                    }
                    Player::X => {
                        println!("Winner: X!");
                    }
                }
            }
            None => (),
        }
    }
}
