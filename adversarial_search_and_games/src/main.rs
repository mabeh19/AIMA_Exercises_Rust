#![allow(dead_code)]
#![allow(unused_imports)]
use std::{
    fmt::Debug,
    thread::sleep
};

use console::Term;
use rand::prelude::*;

pub mod algorithms;

use crate::algorithms::{
    node,
    game::*,
    games::chess,
    minimax,
    alpha_beta,
    monte_carlo_tree_search,
};

type Algorithm<G, S, A> = fn(game: &G, state: &S, depth: usize) -> Option<A>;

const AI_VS_AI: bool = true;
const CHECK_AI_CHOICE: bool = false;

fn main() {
    play_chess();
}


fn try_algorithm<G, S, A, P>(algorithm: Algorithm<G, S, A>, game: &G, state: &S, depth: usize) -> Option<A>
where
    G: Game<S, A, P> + Clone,
    S: Clone + Debug,
    A: Clone + Debug,
    P: Player<S, A>
{
    algorithm(game, state, depth)
}


fn play_chess() {
    let mut game = chess::ChessGame::create_game();
    let init_state = game.get_initial_state().clone();
    let term = Term::stdout();
    draw_board(&term, &init_state);
    const AI_DEPTH: usize = 4;
    let mut state = init_state.clone();
    /*
     * First we perform some sequence of moves to get the game started...
     */
   
    let mut rng = rand::thread_rng();
    for m in chess::OPENERS[rng.gen_range(0..chess::OPENERS.len())] {
        state = game.take_action(&state, &m).clone();
        sleep(std::time::Duration::from_millis(500));
        draw_board(&term, &state);
    }

    let mut mcts = monte_carlo_tree_search::MonteCarloTree::<_, chess::ChessAction>::new(&state);    

    while !game.is_terminal(&state) {
        let current_player = chess::ChessGame::to_move(&state).clone();
        /* 
         * White
         */
        if !AI_VS_AI {
            let moves = game.actions(&state);
            display_actions(&term, &moves);
            let choice: usize = term.read_line().unwrap().trim().parse().unwrap();
            state = game.result(&state, &moves[choice]).clone();
        } else {
            //let choice = try_algorithm(minimax::minimax_search, &game, &state, AI_DEPTH);
            //let choice = try_algorithm(alpha_beta::alpha_beta_search, &game, &state, AI_DEPTH);
            let choice = mcts.monte_carlo_tree_search(&game, &state);
            if CHECK_AI_CHOICE {
                term.move_cursor_to(10, 0).expect("");
                term.write_line(&format!("Choice: {:?}", choice)).expect("");
                term.move_cursor_right(10,).expect("");
                term.write_line(&format!("Current utility of state: {}", game.utility(&state, chess::ChessGame::to_move(&state)))).expect("");
                term.read_key().expect("");
            }
            if choice.is_some() {
                state = game.result(&state, &choice.unwrap()).clone();
            } else {
                println!("Game over, {:?} wins!", chess::ChessGame::to_move(&state).get_color());
                break;
            }
        }
        draw_board(&term, &state);
        term.move_cursor_down(1).expect("");
        term.write_line(&format!("Current utility for {:?} = {:.4}", current_player.get_color(), game.utility(&state, &current_player))).expect("");
        /*
         * Black
         */
        //let choice = try_algorithm(minimax::minimax_search, &game, &state, 4);
        /*
        let choice = if state.2 < 8 {
            Some(chess::OPENERS[0][state.2])
        } else {
            try_algorithm(minimax::minimax_search, &game, &state, 4)
        };*/
        //if choice.is_some() {
        //    state = game.take_action(&state, &choice.unwrap()).clone();
        //}
        //sleep(std::time::Duration::from_millis(100));
        
        //draw_board(&term, &state);
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
    term.clear_screen().expect("");
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
                                icon = "\x1b[38;5;11mK\x1b[38;5;0m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mK\x1b[38;5;0m";
                            }
                        }
                    },
                    chess::ChessPieceType::Queen => {
                        match p.get_color() {
                            chess::PlayerColor::White => {
                                icon = "\x1b[38;5;11mQ\x1b[38;5;0m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mQ\x1b[38;5;0m";
                            }
                        }
                    },
                    chess::ChessPieceType::Rook => {
                        match p.get_color() {
                            chess::PlayerColor::White => {
                                icon = "\x1b[38;5;11mR\x1b[38;5;0m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mR\x1b[38;5;0m";
                            }
                        }
                    },
                    chess::ChessPieceType::Knight => {
                        match p.get_color() {
                            chess::PlayerColor::White => {
                                icon = "\x1b[38;5;11mN\x1b[38;5;0m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mN\x1b[38;5;0m";
                            }
                        }
                    },
                    chess::ChessPieceType::Bishop => {
                        match p.get_color() {
                            chess::PlayerColor::White => {
                                icon = "\x1b[38;5;11mB\x1b[38;5;0m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mB\x1b[38;5;0m";
                            }
                        }
                    },
                    chess::ChessPieceType::Pawn => {
                        match p.get_color() {
                            chess::PlayerColor::White => {
                                icon = "\x1b[38;5;11mP\x1b[38;5;0m";
                            },
                            chess::PlayerColor::Black => {
                                icon = "\x1b[38;5;5mP\x1b[38;5;0m";
                            }
                        }
                    }
                }
                term.write_line(icon).expect("");
            }
        }
    }
}
