
use std::collections::HashMap;

use crate::algorithms::game::{Game, Player};

type BoardPosition = (isize, isize);
type ChessState = [[Option<Box<ChessPiece>>; 8]; 8];
type ChessAction<'a> = (&'a Box<ChessPiece>, BoardPosition);

const BOARD_ROWS: isize = 8;
const BOARD_COLS: isize = 8;
const KING_VALUE: f64 = f64::MAX;
const QUEEN_VALUE: f64 = 9.;
const ROOK_VALUE: f64 = 5.;
const KNIGHT_VALUE: f64 = 3.;
const BISHOP_VALUE: f64 = 3.;
const PAWN_VALUE: f64 = 1.;

const PIECE_VALUES: [f64; 6] = [
    KING_VALUE,
    QUEEN_VALUE,
    ROOK_VALUE,
    KNIGHT_VALUE,
    BISHOP_VALUE,
    PAWN_VALUE
];

macro_rules! array {
    (@accum (0, $($_es:expr),*) -> ($($body:tt)*))
        => {array!(@as_expr [$($body)*])};
    (@accum (1, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (0, $($es),*) -> ($($body)* $($es,)*))};
    (@accum (2, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (0, $($es),*) -> ($($body)* $($es,)* $($es,)*))};
    (@accum (3, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (2, $($es),*) -> ($($body)* $($es,)*))};
    (@accum (4, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (2, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (5, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)*))};
    (@accum (6, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)* $($es,)*))};
    (@accum (7, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)* $($es,)* $($es,)*))};
    (@accum (8, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (16, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (8, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (32, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (16, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (64, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (32, $($es,)* $($es),*) -> ($($body)*))};

    (@as_expr $e:expr) => {$e};

    [$e:expr; $n:tt] => { array!(@accum ($n, $e) -> ()) };
}

pub struct ChessGame {
    board: ChessState,
    players: [ChessPlayer; 2],
    ply: usize,
    game_over: bool,
}

impl ChessGame {
    fn get_current_player(&self) -> &ChessPlayer {
        &self.players[self.ply % 2]
    }

    fn move_piece<'a>(&mut self, action: ChessAction<'a>) {
        Self::perform_move(&mut self.board, &action);
        self.ply += 1;
    }

    fn is_legal_move<'a>(state: &ChessState, action: ChessAction<'a>) -> bool {
        let cur_pos = action.0.get_position();
        
        if action.1.0 > 7 || action.1.1 > 7 {
            return false;
        }

        let square_value = &state[cur_pos.0 as usize][cur_pos.1 as usize];
        match square_value {
            Some(piece) => {
                // Compare color of piece at destination and color of moving piece
                piece.get_color() != action.0.get_color()
            },
            None => {
                true
            }
        }
    }

    fn hypothetical_move<'a>(state: &ChessState, action: &ChessAction<'a>) -> ChessState {
        let mut new_state = state.clone();
        
        Self::perform_move(&mut new_state, &action);

        new_state
    }

    fn perform_move<'a>(state: &mut ChessState, action: &ChessAction<'a>) {
        let cur_pos = action.0.get_position();
        state[cur_pos.0 as usize][cur_pos.1 as usize] = None;
        state[action.1.0 as usize][action.1.0 as usize] = Some(action.0.clone());
    }
}

impl<'a> Game<ChessState, ChessAction<'a>, ChessPlayer> for ChessGame {
    fn create_game() -> Self {
        let mut new_game = Self {
            board: array![array![None; 8]; 8],
            players: [ChessPlayer::new(PlayerColor::White),
                      ChessPlayer::new(PlayerColor::Black)],
            ply: 0,
            game_over: false
        };

        new_game
    }

    fn get_initial_state(&self) -> &ChessState {
        &self.board
    }

    fn to_move(&self, state: &ChessState) -> &ChessPlayer {
        &self.players[self.ply % 2]
    }

    fn actions(&self, state: &ChessState) -> Vec<ChessAction<'a>> {       
        self.get_current_player().get_moves(state)
    }

    fn result(&self, state: &ChessState, action: &ChessAction<'a>) -> ChessState {
        ChessGame::hypothetical_move(state, action)        
    }

    fn is_terminal(&self, state: &ChessState) -> bool {
        self.game_over
    }

    fn utility(&self, state: &ChessState, player: &ChessPlayer) -> f64 {
        
        0.
    }
}

pub struct ChessPlayer {
    king: Box<ChessPiece>,
    queens: Vec<Box<ChessPiece>>,
    rooks: Vec<Box<ChessPiece>>,
    knights: Vec<Box<ChessPiece>>,
    bishops: Vec<Box<ChessPiece>>,
    pawns: Vec<Box<ChessPiece>>,
    is_checked: bool
}

impl<'a> Player<ChessState, ChessAction<'a>> for ChessPlayer {

}

#[derive(PartialEq, Eq, Clone, Copy)]
enum PlayerColor {
    White,
    Black
}

impl ChessPlayer {
    
    fn new(color: PlayerColor) -> Self {
        let new_player: ChessPlayer;
        let king: ChessPiece;
        let queen: ChessPiece;
        let rooks: Vec<Box<ChessPiece>>;
        let knights: Vec<Box<ChessPiece>>;
        let bishops: Vec<Box<ChessPiece>>;
        let mut pawns: Vec<Box<ChessPiece>>;

 
        match color {
            PlayerColor::White => {
                king = ChessPiece { piece_type: ChessPieceType::King, position: (4, 0), color };
                queen = ChessPiece { piece_type: ChessPieceType::Queen, position: (3, 0), color };
                rooks = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Rook, position: (0, 0), color }), Box::new(ChessPiece { piece_type: ChessPieceType::Rook, position: (0, 7), color })];
                knights = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Knight, position: (1, 0), color }), Box::new(ChessPiece { piece_type: ChessPieceType::Knight, position: (6, 0), color })];
                bishops = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Bishop, position: (2, 0), color }), Box::new(ChessPiece { piece_type: ChessPieceType::Bishop, position: (5, 0), color })];
                pawns = Vec::new();
                for i in 0..BOARD_COLS {
                    pawns.push(Box::new(ChessPiece { piece_type: ChessPieceType::Pawn, position: (i, 1), color }));
                }
            },
            PlayerColor::Black => {
                king = ChessPiece { piece_type: ChessPieceType::King, position: (4, 7), color };
                queen = ChessPiece { piece_type: ChessPieceType::Queen, position: (3, 7), color };
                rooks = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Rook, position: (0, 7), color }), Box::new(ChessPiece { piece_type: ChessPieceType::Rook, position: (7, 7), color })];
                knights = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Knight, position: (1, 7), color }), Box::new(ChessPiece { piece_type: ChessPieceType::Knight, position: (6, 7), color })];
                bishops = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Bishop, position: (2, 7), color }), Box::new(ChessPiece { piece_type: ChessPieceType::Bishop, position: (5, 7), color })];
                pawns = Vec::new();
                for i in 0..BOARD_COLS {
                    pawns.push(Box::new(ChessPiece { piece_type: ChessPieceType::Pawn, position: (i, 6), color }));
                }
            }
        }
               
        
        new_player = Self {
            king: Box::new(king),
            queens: vec![Box::new(queen)],
            rooks,
            knights,
            bishops,
            pawns,
            is_checked: false
        };
        
        new_player
    }

    fn get_moves<'a>(&self, state: &ChessState) -> Vec<ChessAction<'a>> {
        let mut all_moves: Vec<ChessAction<'a>>;

        for m in self.king.get_possible_moves(state) {
            all_moves.push((&self.king, m));
        }
        for queen in self.queens {
            for m in queen.get_possible_moves(state) {
                all_moves.push((&queen, m));
            }
        }
        for rook in self.rooks {
            for m in rook.get_possible_moves(state) {
                all_moves.push((&rook, m));
            }
        }
        for knight in self.knights {
            for m in knight.get_possible_moves(state) {
                all_moves.push((&knight, m));
            }
       }
        for bishop in self.bishops {
            for m in bishop.get_possible_moves(state) {
                all_moves.push((&bishop, m));
            }
        }
        for pawn in self.pawns {
            for m in pawn.get_possible_moves(state) {
                all_moves.push((&pawn, m));
            }
        }

        all_moves
    }
}

#[derive(Clone, Copy)]
struct ChessPiece {
    piece_type: ChessPieceType,
    position: BoardPosition,
    color: PlayerColor
}

impl ChessPiece {
    fn get_possible_moves<'a>(&self, state: &ChessState) -> Vec<BoardPosition> {
        let mut moves: Vec<BoardPosition> = Vec::new();
        let vectors: Vec<BoardPosition>;
        let range: isize;
        match self.piece_type {
            ChessPieceType::King => {
                vectors = vec![
                    (1, 0),
                    (1, 1),
                    (1, -1),
                    (-1, 0),
                    (-1, 1),
                    (-1, -1),
                    (0, 1),
                    (0, -1)
                ];
                range = 1;
            },
            ChessPieceType::Queen => {
                vectors = vec![
                    (1, 0),
                    (1, 1),
                    (1, -1),
                    (-1, 0),
                    (-1, 1),
                    (-1, -1),
                    (0, 1),
                    (0, -1)
                ];
                range = 8;
            },
            ChessPieceType::Rook => {
                vectors = vec![
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1)
                ];
                range = 8;
            },
            ChessPieceType::Knight => {
                vectors = vec![
                    (1, 2),
                    (1, -2),
                    (-1, 2),
                    (-1, -2),
                    (2, 1),
                    (2, -1),
                    (-2, 1),
                    (-2, -2)
                ];
                range = 1;
            },
            ChessPieceType::Bishop => {
                vectors = vec![
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1)
                ];
                range = 8;
            },
            ChessPieceType::Pawn => {
                vectors = vec![
                    (0,1),
                    (1,1),
                    (-1,1),
                    (0,2)
                ];
                range = 1;
            }
        }

        for vector in vectors {
            let mut new_pos = self.position;
            for _ in 0..range {
                new_pos = (new_pos.0 + vector.0, new_pos.1 + vector.1);
                let action: ChessAction = (&Box::new(self.clone()), new_pos);
                if ChessGame::is_legal_move(state, action) {
                    moves.push(new_pos);
                }
            }
        }

        moves
    }
    
    fn get_value(&self) -> f64 {
        self.piece_type.get_value()
    }

    fn get_position(&self) -> BoardPosition {
        self.position
    }

    fn get_color(&self) -> PlayerColor {
        self.color
    }
}

#[derive(Clone, Copy)]
enum ChessPieceType {
    King = 0,
    Queen = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Pawn = 5
}

impl ChessPieceType {

    fn get_value(&self) -> f64 {
        PIECE_VALUES[*self as usize]
    }
}

