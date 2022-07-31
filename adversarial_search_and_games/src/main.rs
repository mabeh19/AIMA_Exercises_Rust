use std::fmt::Debug;

use console::Term;

pub mod algorithms;

use crate::algorithms::{
    game::*,
    games::chess,
    minimax,
};


type Algorithm<G, S, A>= fn(game: G, state: &S, depth: usize) -> Option<A>;


fn main() {
    try_algorithm::<chess::ChessGame, _, _, _>(minimax::minimax_search, 2);
}


fn try_algorithm<G, S, A, P>(algorithm: Algorithm<G, S, A>, depth: usize)
where
    G: Game<S, A, P> + Clone,
    S: Clone + Debug,
    A: Clone + Debug,
    P: Player<S, A>
{
    let game: G = Game::create_game();
    let init_state = game.get_initial_state().clone();
    let optimal_move = algorithm(game, &init_state, depth);
    println!("Optimal action = {:?}", optimal_move);
}


fn play_chess() {
    let mut game = chess::ChessGame::create_game();
    let init_state = game.get_initial_state().clone();
    let term = Term::stdout();
    
    
    let mut state = init_state.clone();
    while !game.is_terminal(&state) {
        let moves = game.actions(&state);
       
        /* Display current state and actions */
        term.clear_screen().expect("");
        draw_board(&term, &state);
        display_actions(&term, &moves);
        
        let choice: usize = term.read_line().unwrap().trim().parse().unwrap();
        state = game.take_action(&state, &moves[choice]).clone();
    }
    

}

fn display_actions(term: &Term, actions: &Vec<chess::ChessAction>) {
    let pos = (10, 0);
    let mut x = 0;
    term.move_cursor_to(pos.0, pos.1).expect("");
    for action in actions {
        let line = format!("[{:0>2}] {:?}->{:?}", x, action.0, action.1);
        term.write_line(&line).expect("");
        term.move_cursor_right(pos.0).expect("");
        x += 1;
    }
}

fn draw_board(term: &Term, state: &chess::ChessState) {
    for row in &state.0 {
        for piece in row {
            if let Some(p) = piece {
                let pos = p.get_position();
                let icon: &'static str;
                term.move_cursor_to(pos.0 as usize, pos.1 as usize).expect("");
                match p.get_type() {
                    chess::ChessPieceType::King => {
                        match p.get_color() {
                            chess::PlayerColor::White => {
                                icon = "\x1b[38;5;11mK\x1b[38;5;15m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mK\x1b[38;5;15m";
                            }
                        }
                    },
                    chess::ChessPieceType::Queen => {
                        match p.get_color() {
                            chess::PlayerColor::White => {
                                icon = "\x1b[38;5;11mQ\x1b[38;5;15m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mQ\x1b[38;5;15m";
                            }
                        }
                    },
                    chess::ChessPieceType::Rook => {
                        match p.get_color() {
                            chess::PlayerColor::White => {
                                icon = "\x1b[38;5;11mR\x1b[38;5;15m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mR\x1b[38;5;15m";
                            }
                        }
                    },
                    chess::ChessPieceType::Knight => {
                        match p.get_color() {
                            chess::PlayerColor::White => {
                                icon = "\x1b[38;5;11mN\x1b[38;5;15m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mN\x1b[38;5;15m";
                            }
                        }
                    },
                    chess::ChessPieceType::Bishop => {
                        match p.get_color() {
                            chess::PlayerColor::White => {
                                icon = "\x1b[38;5;11mB\x1b[38;5;15m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mB\x1b[38;5;15m";
                            }
                        }
                    },
                    chess::ChessPieceType::Pawn => {
                        match p.get_color() {
                            chess::PlayerColor::White => {
                                icon = "\x1b[38;5;11mP\x1b[38;5;15m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mP\x1b[38;5;15m";
                            }
                        }
                    }
                }
                term.write_line(icon).expect("");
            }
        }
    }
}
