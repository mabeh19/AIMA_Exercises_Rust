
use crate::algorithms::game::{Game, Player};

type BoardPosition = (isize, isize);
type ChessState = [[Option<Box<ChessPiece>>; 8]; 8];
type ChessAction = (BoardPosition/*&'a Box<ChessPiece>*/, BoardPosition, bool);

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

    fn move_piece(&mut self, action: ChessAction) {
        Self::perform_move(&mut self.board, &action);
        self.ply += 1;
    }

    fn is_legal_move(state: &ChessState, action: ChessAction) -> bool {
        let cur_pos = action.0;//.get_position();
        let piece = &state[cur_pos.0 as usize][cur_pos.1 as usize];
        if action.1.0 > 7 || action.1.1 > 7 {
            return false;
        }

        let square_value = &state[cur_pos.0 as usize][cur_pos.1 as usize];
        match square_value {
            Some(piece) => {
                // Compare color of piece at destination and color of moving piece
                piece.get_color() != piece.get_color()
            },
            None => {
                true
            }
        }
    }

    fn hypothetical_move(state: &ChessState, action: &ChessAction) -> ChessState {
        let mut new_state = state.clone();
        
        Self::perform_move(&mut new_state, &action);

        new_state
    }

    fn perform_move(state: &mut ChessState, action: &ChessAction) {
        let cur_pos = action.0;
        let piece = state[cur_pos.0 as usize][cur_pos.1 as usize].clone();
        state[action.1.0 as usize][action.1.0 as usize] = Some(piece.unwrap());
        state[cur_pos.0 as usize][cur_pos.1 as usize] = None;

    }
}

impl Game<ChessState, ChessAction, ChessPlayer> for ChessGame {
    fn create_game() -> Self {
        let new_game = Self {
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

    fn actions(&self, state: &ChessState) -> Vec<ChessAction> {       
        self.get_current_player().get_moves(state)
    }

    fn result(&self, state: &ChessState, action: &ChessAction) -> ChessState {
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
    checked_by: Option<ChessPiece>
}

impl Player<ChessState, ChessAction> for ChessPlayer {

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
            checked_by: None
        };
        
        new_player
    }

    fn get_moves(&self, state: &ChessState) -> Vec<ChessAction> {
        let mut all_moves: Vec<ChessAction> = Vec::new();
        
        if self.checked_by.is_some() {
            

            return all_moves;
        }

        for m in self.king.get_possible_moves(state) {
            all_moves.push((self.king.get_position(), m, false));
        }
        for queen in &self.queens {
            for m in queen.get_possible_moves(state) {
                let mut is_check: bool = false;
                if state[m.0 as usize][m.1 as usize].is_some() {
                    is_check = state[m.0 as usize][m.1 as usize].as_ref().unwrap().get_type() == ChessPieceType::King;
                }
                all_moves.push((queen.get_position(), m, is_check));
            }
        }
        for rook in &self.rooks {
            for m in rook.get_possible_moves(state) {
                let mut is_check: bool = false;
                if state[m.0 as usize][m.1 as usize].is_some() {
                    is_check = state[m.0 as usize][m.1 as usize].as_ref().unwrap().get_type() == ChessPieceType::King;
                }
                all_moves.push((rook.get_position(), m, is_check));
            }
        }
        for knight in &self.knights {
            for m in knight.get_possible_moves(state) {
                let mut is_check: bool = false;
                if state[m.0 as usize][m.1 as usize].is_some() {
                    is_check = state[m.0 as usize][m.1 as usize].as_ref().unwrap().get_type() == ChessPieceType::King;
                }
                all_moves.push((knight.get_position(), m, is_check));
            }
        }
        for bishop in &self.bishops {
            for m in bishop.get_possible_moves(state) {
                let mut is_check: bool = false;
                if state[m.0 as usize][m.1 as usize].is_some() {
                    is_check = state[m.0 as usize][m.1 as usize].as_ref().unwrap().get_type() == ChessPieceType::King;
                }
                all_moves.push((bishop.get_position(), m, is_check));
            }
        }
        for pawn in &self.pawns {
            for m in pawn.get_possible_moves(state) {
                let mut is_check: bool = false;
                if state[m.0 as usize][m.1 as usize].is_some() {
                    is_check = state[m.0 as usize][m.1 as usize].as_ref().unwrap().get_type() == ChessPieceType::King;
                }
                all_moves.push((pawn.get_position(), m, is_check));
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
    fn get_possible_moves(&self, state: &ChessState) -> Vec<BoardPosition> {
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
                let action: ChessAction = (self.position, new_pos, false);
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

    fn get_type(&self) -> ChessPieceType {
        self.piece_type
    }
}

#[derive(Clone, Copy, PartialEq)]
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

