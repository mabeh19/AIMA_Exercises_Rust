use std::collections::HashMap;

use crate::algorithms::game::{Game, Player};


pub type BoardPosition = (isize, isize);
pub type ChessState = ([[Option<Box<ChessPiece>>; 8]; 8], [ChessPlayer; 2], usize);
pub type ChessAction = (BoardPosition, BoardPosition, bool);

//const BOARD_ROWS: isize = 8;
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

pub const OPENERS: [[ChessAction; 8]; 1] = [
    /* Scandinavian */
    [((4,6), (4,4), false), ((3,1), (3,3), false), // 1: e4 d5
     ((4,4), (3,3), false), ((3,0), (3,3), false), // 2: d5 Qd5
     ((6,7), (5,5), false), ((2,0), (6,4), false), // 3: Nf3 Bg4
     ((5,7), (4,6), false), ((1,0), (2,2), false)] // 4: Be2 Nc6
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

#[derive(Clone, Debug)]
pub struct ChessGame {
    board: ChessState,
}

impl ChessGame {
    fn get_current_player(state: &ChessState) -> &ChessPlayer {
        &state.1[state.2 % 2]
    }

    fn get_current_player_as_mut(state: &mut ChessState) -> &mut ChessPlayer {
        &mut state.1[state.2 % 2]
    }
    
    fn get_other_player(state: &ChessState) -> &ChessPlayer {
        &state.1[(state.2 + 1) % 2]
    }

    fn get_other_player_as_mut(state: &mut ChessState) -> &mut ChessPlayer {
        &mut state.1[(state.2 + 1) % 2]
    }

    fn move_piece(&mut self, action: ChessAction) {
        let piece_taken = Self::perform_move(&mut self.board, &action);
        // update players piece
        Self::get_current_player_as_mut(&mut self.board).update_piece(&action);
        if piece_taken {
            Self::get_other_player_as_mut(&mut self.board).remove_piece(&action);
        }
        self.board.2 += 1;
    }

    fn is_legal_move(state: &ChessState, action: ChessAction) -> bool {
        let cur_pos = action.0;
        let new_pos = action.1;
        if action.1.0 as usize > 7 || action.1.1 as usize > 7 {
            return false;
        }
        let piece = &state.0[cur_pos.1 as usize][cur_pos.0 as usize];

        let square_value = &state.0[new_pos.1 as usize][new_pos.0 as usize];
        match square_value {
            Some(other_piece) => {
                // Compare color of piece at destination and color of moving piece
                piece.as_ref().unwrap().get_color() != other_piece.get_color()
            },
            None => {
                true
            }
        }
    }

    fn contains_opponent_piece(state: &ChessState, action: &ChessAction) -> bool {
        let cur_pos = action.0;
        let new_pos = action.1;
        if action.1.0 as usize > 7 || action.1.1 as usize > 7 {
            return false;
        }
        let piece = &state.0[cur_pos.1 as usize][cur_pos.0 as usize];
        let square_value = &state.0[new_pos.1 as usize][new_pos.0 as usize];
        match square_value {
            Some(other_piece) => {
                // Compare color of piece at destination and color of moving piece
                piece.as_ref().unwrap().get_color() != other_piece.get_color()
            },
            None => {
                false
            }
        }
    }

    fn hypothetical_move(&mut self, state: &ChessState, action: &ChessAction) -> ChessState {
        let mut new_state = state.clone();
        
        let piece_taken = Self::perform_move(&mut new_state, &action);
        // update players piece
        Self::get_current_player_as_mut(&mut new_state).update_piece(&action);
        if piece_taken {
            Self::get_other_player_as_mut(&mut new_state).remove_piece(&action);
        }
        new_state.2 += 1;

        new_state
    }

    fn perform_move(state: &mut ChessState, action: &ChessAction) -> bool {
        let cur_pos = action.0;
        let new_pos = action.1;
        let piece_taken = state.0[new_pos.1 as usize][new_pos.0 as usize].is_some();
        let mut piece = state.0[cur_pos.1 as usize][cur_pos.0 as usize].as_mut().unwrap();
        // update piece's own state
        piece.position = action.1; 
        state.0[new_pos.1 as usize][new_pos.0 as usize] = Some(piece.clone());
        state.0[cur_pos.1 as usize][cur_pos.0 as usize] = None;

        piece_taken
    }
}

impl Game<ChessState, ChessAction, ChessPlayer> for ChessGame {
    fn create_game() -> Self {
        let mut new_game = Self {
            board: (array![array![None; 8]; 8], [ChessPlayer::new(PlayerColor::White),
                      ChessPlayer::new(PlayerColor::Black)], 0),
        };

        for p in &new_game.board.1 {
            // place king
            new_game.board.0[p.king.as_ref().unwrap().get_position().1 as usize][p.king.as_ref().unwrap().get_position().0 as usize] = p.king.clone();
        
            // place queen(s?)
            for q in &p.queens {
                let pos = q.get_position();
                new_game.board.0[pos.1 as usize][pos.0 as usize] = Some(q.clone());
            }

            // place rooks
            for r in &p.rooks {
                let pos = r.get_position();
                new_game.board.0[pos.1 as usize][pos.0 as usize] = Some(r.clone());
            }

            // place knights
            for n in &p.knights {
                let pos = n.get_position();
                new_game.board.0[pos.1 as usize][pos.0 as usize] = Some(n.clone());
            }

            // place bishops
            for b in &p.bishops {
                let pos = b.get_position();
                new_game.board.0[pos.1 as usize][pos.0 as usize] = Some(b.clone());
            }

            // place pawns
            for pawn in &p.pawns {
                let pos = pawn.get_position();
                new_game.board.0[pos.1 as usize][pos.0 as usize] = Some(pawn.clone());
            }
        }


        new_game
    }

    fn get_initial_state(&self) -> &ChessState {
        &self.board
    }

    fn to_move(state: &ChessState) -> &ChessPlayer {
        Self::get_current_player(state)
    }

    fn actions(&self, state: &ChessState) -> Vec<ChessAction> {       
        Self::get_current_player(state).get_moves(state)
    }

    fn result(&mut self, state: &ChessState, action: &ChessAction) -> ChessState {
        self.hypothetical_move(state, action)
    }

    fn is_terminal(&self, state: &ChessState) -> bool {
        // If we can't perform any moves, the game must be over
        if Self::get_current_player(state).get_moves(state).len() == 0 {
            true
        } else {
            false
        }
    }

    fn utility(&self, state: &ChessState, player: &ChessPlayer) -> f64 {
        // eval function from https://www.chessprogramming.org/Evaluation#Where_to_Start
        let other_player = Self::get_other_player(state);
        let k = if player.king.is_some() { 1. } else { 0. };
        let k_m = if other_player.king.is_some() { 1. } else { 0. };
        let utility = 200. * (k - k_m)
        + 9. * (player.queens.len() as f64 - other_player.queens.len() as f64)
        + 5. * (player.rooks.len() as f64 - other_player.rooks.len() as f64)
        + 3. * (player.knights.len() as f64 - other_player.knights.len() as f64 + player.bishops.len() as f64 - other_player.bishops.len() as f64)
        + 1. * (player.pawns.len() as f64 - other_player.pawns.len() as f64)
        +0.1 * (player.get_moves(state).len() as f64 - other_player.get_moves(state).len() as f64);
        return utility;
    }

    fn take_action(&mut self, _state: &ChessState, action: &ChessAction) -> &ChessState {
        self.move_piece(action.clone());
        &self.board
    }
}

#[derive(Clone, Debug)]
pub struct ChessPlayer {
    color: PlayerColor,
    pieces: HashMap<(isize, isize), (ChessPieceType, usize)>,
    king: Option<Box<ChessPiece>>,
    queens: Vec<Box<ChessPiece>>,
    rooks: Vec<Box<ChessPiece>>,
    knights: Vec<Box<ChessPiece>>,
    bishops: Vec<Box<ChessPiece>>,
    pawns: Vec<Box<ChessPiece>>,
    checked_by: Option<ChessPiece>
}

impl Player<ChessState, ChessAction> for ChessPlayer {

}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PlayerColor {
    White,
    Black
}

impl ChessPlayer {
    
    fn new(color: PlayerColor) -> Self {
        let mut new_player: ChessPlayer;
        let king: Box<ChessPiece>;
        let queen: Box<ChessPiece>;
        let rooks: Vec<Box<ChessPiece>>;
        let knights: Vec<Box<ChessPiece>>;
        let bishops: Vec<Box<ChessPiece>>;
        let mut pawns: Vec<Box<ChessPiece>>;
        
        match color {
            PlayerColor::White => {
                king = Box::new(ChessPiece { piece_type: ChessPieceType::King, position: (4, 7 - 0), color, can_perform: true });
                queen = Box::new(ChessPiece { piece_type: ChessPieceType::Queen, position: (3, 7 - 0), color, can_perform: false });
                rooks = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Rook, position: (0, 7 - 0), color, can_perform: true }), Box::new(ChessPiece { piece_type: ChessPieceType::Rook, position: (7, 7 - 0), color, can_perform: true })];
                knights = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Knight, position: (1, 7 - 0), color, can_perform: false }), Box::new(ChessPiece { piece_type: ChessPieceType::Knight, position: (6, 7 - 0), color, can_perform: false })];
                bishops = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Bishop, position: (2, 7 - 0), color, can_perform: false }), Box::new(ChessPiece { piece_type: ChessPieceType::Bishop, position: (5, 7 - 0), color, can_perform: false })];
                pawns = Vec::new();
                for i in 0..BOARD_COLS {
                    pawns.push(Box::new(ChessPiece { piece_type: ChessPieceType::Pawn, position: (i, 7 - 1), color, can_perform: true }));
                }
            },
            PlayerColor::Black => {
                king = Box::new(ChessPiece { piece_type: ChessPieceType::King, position: (4, 7 - 7), color, can_perform: true });
                queen = Box::new(ChessPiece { piece_type: ChessPieceType::Queen, position: (3, 7 - 7), color, can_perform: false });
                rooks = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Rook, position: (0, 7 - 7), color, can_perform: true }), Box::new(ChessPiece { piece_type: ChessPieceType::Rook, position: (7, 7 - 7), color, can_perform: true })];
                knights = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Knight, position: (1, 7 - 7), color, can_perform: false }), Box::new(ChessPiece { piece_type: ChessPieceType::Knight, position: (6, 7 - 7), color, can_perform: false })];
                bishops = vec![Box::new(ChessPiece { piece_type: ChessPieceType::Bishop, position: (2, 7 - 7), color, can_perform: false }), Box::new(ChessPiece { piece_type: ChessPieceType::Bishop, position: (5, 7 - 7), color, can_perform: false })];
                pawns = Vec::new();
                for i in 0..BOARD_COLS {
                    pawns.push(Box::new(ChessPiece { piece_type: ChessPieceType::Pawn, position: (i, 7 - 6), color, can_perform: true }));
                }
            }
        }
        
        
        new_player = Self {
            color, 
            pieces: HashMap::new(),
            king: Some(king),
            queens: vec![queen],
            rooks,
            knights,
            bishops,
            pawns,
            checked_by: None
        };

        new_player.pieces.insert(new_player.king.as_ref().unwrap().position, (new_player.king.as_ref().unwrap().piece_type, 0));
        new_player.pieces.insert(new_player.queens[0].position, (new_player.queens[0].piece_type, 0));
        let mut i = 0;
        for rook in &new_player.rooks {
            new_player.pieces.insert(rook.position, (rook.piece_type, i));
            i += 1;
        }
        i = 0;
        for knight in &new_player.knights {
            new_player.pieces.insert(knight.position, (knight.piece_type, i));
            i += 1;
        }
        i = 0;
        for bishop in &new_player.bishops {
            new_player.pieces.insert(bishop.position, (bishop.piece_type, i));
            i += 1;
        }
        i = 0;
        for pawn in &new_player.pawns {
            new_player.pieces.insert(pawn.position, (pawn.piece_type, i));
            i += 1;
        }
        
        new_player
    }

    fn get_moves(&self, state: &ChessState) -> Vec<ChessAction> {
        let mut all_moves: Vec<ChessAction> = Vec::new();
        let mut checked_squares: Vec<(isize, isize)> = Vec::new();

        if self.checked_by.is_some() {
            let other_pos = self.checked_by.as_ref().unwrap().get_position();
            let king_pos = self.king.as_ref().unwrap().get_position();
            let max_length = (other_pos.0-king_pos.0).max(other_pos.1-king_pos.1);
            let diff = (other_pos.0 - king_pos.0, other_pos.1 - king_pos.1);
            match self.checked_by.as_ref().unwrap().get_type() {
                ChessPieceType::Knight => {
                    /* We handle knights differently, as they aren't stopped by LOS */
                    checked_squares.push(other_pos);
                },
                _ => {
                    /* All pieces but knights */
                    for i in 1..=max_length {
                        let offset = (( diff.0 / max_length ) * i, (diff.1 / max_length) * i);
                        let new_pos = (king_pos.0 + offset.0, king_pos.1 + offset.0);
                        checked_squares.push(new_pos);
                    }
                }
            }
        }
        
        for m in self.king.as_ref().unwrap().get_possible_moves(state) {
            if !checked_squares.contains(&m) {
                all_moves.push((self.king.as_ref().unwrap().get_position(), m, false)); // If the move puts us outside of checked line
            }
        }
        for queen in &self.queens {
            for m in queen.get_possible_moves(state) {
                if self.checked_by.is_none() || checked_squares.contains(&m) {
                    let mut is_check: bool = false;
                    if state.0[m.1 as usize][m.0 as usize].is_some() {
                        is_check = state.0[m.1 as usize][m.0 as usize].as_ref().unwrap().get_type() == ChessPieceType::King;
                    }
                    all_moves.push((queen.get_position(), m, is_check));
                }
            }
        }
        for rook in &self.rooks {
            for m in rook.get_possible_moves(state) {
                if self.checked_by.is_none() || checked_squares.contains(&m) {
                    let mut is_check: bool = false;
                    if state.0[m.1 as usize][m.0 as usize].is_some() {
                        is_check = state.0[m.1 as usize][m.0 as usize].as_ref().unwrap().get_type() == ChessPieceType::King;
                    }
                    all_moves.push((rook.get_position(), m, is_check));
                }
            }
        }
        for knight in &self.knights {
            for m in knight.get_possible_moves(state) {
                    if self.checked_by.is_none() || checked_squares.contains(&m) {
                    let mut is_check: bool = false;
                    if state.0[m.1 as usize][m.0 as usize].is_some() {
                        is_check = state.0[m.1 as usize][m.0 as usize].as_ref().unwrap().get_type() == ChessPieceType::King;
                    }
                    all_moves.push((knight.get_position(), m, is_check));
                    }
            }
        }
        for bishop in &self.bishops {
            for m in bishop.get_possible_moves(state) {
                if self.checked_by.is_none() || checked_squares.contains(&m) {
                    let mut is_check: bool = false;
                    if state.0[m.1 as usize][m.0 as usize].is_some() {
                        is_check = state.0[m.1 as usize][m.0 as usize].as_ref().unwrap().get_type() == ChessPieceType::King;
                    }
                    all_moves.push((bishop.get_position(), m, is_check));
                }
            }
        }
        for pawn in &self.pawns {
            for m in pawn.get_possible_moves(state) {
                if self.checked_by.is_none() || checked_squares.contains(&m) {
                    let mut is_check: bool = false;
                    if state.0[m.1 as usize][m.0 as usize].is_some() {
                        is_check = state.0[m.1 as usize][m.0 as usize].as_ref().unwrap().get_type() == ChessPieceType::King;
                    }
                    all_moves.push((pawn.get_position(), m, is_check));
                }
            }
        }

        all_moves
    }

    pub fn get_color(&self) -> PlayerColor {
        self.color
    }

    fn update_piece(&mut self, action: &ChessAction) {
        let piece_info = *self.pieces.get(&action.0).unwrap();
        match piece_info.0 {
            ChessPieceType::King => {
                self.king.as_mut().unwrap().position = action.1;
            },
            ChessPieceType::Queen => {
                self.queens[piece_info.1].position = action.1;
            },
            ChessPieceType::Rook => {
                self.rooks[piece_info.1].position = action.1;
            },
            ChessPieceType::Knight => {
                self.knights[piece_info.1].position = action.1;
            },
            ChessPieceType::Bishop => {
                self.bishops[piece_info.1].position = action.1;
            },
            ChessPieceType::Pawn => {
                self.pawns[piece_info.1].position = action.1;
                self.pawns[piece_info.1].can_perform = false;
            }
        }
        self.pieces.remove(&action.0);
        self.pieces.insert(action.1, piece_info);
    }

    fn remove_piece(&mut self, action: &ChessAction) {
        let piece_info = *self.pieces.get(&action.1).unwrap();
        match piece_info.0 {
            ChessPieceType::King => {
                self.king = None;   
            },
            ChessPieceType::Queen => {
                self.queens.remove(piece_info.1);
            },
            ChessPieceType::Rook => {
                self.rooks.remove(piece_info.1);
            },
            ChessPieceType::Knight => {
                self.knights.remove(piece_info.1);
            },
            ChessPieceType::Bishop => {
                self.bishops.remove(piece_info.1);
            },
            ChessPieceType::Pawn => {
                self.pawns.remove(piece_info.1);
            }
        }
        self.pieces.remove(&action.1);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ChessPiece {
    piece_type: ChessPieceType,
    position: BoardPosition,
    color: PlayerColor,
    can_perform: bool // this is used for initial movement of pawns and castling of kings
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

        for vector in &vectors {
            let mut new_pos = self.position;
            for _ in 0..range {
                if self.color == PlayerColor::White {
                    new_pos = (new_pos.0 + vector.0, new_pos.1 - vector.1);
                } else {
                    new_pos = (new_pos.0 + vector.0, new_pos.1 + vector.1);
                }
                let action: ChessAction = (self.position, new_pos, false);
                
                if self.piece_type == ChessPieceType::Pawn && (new_pos.0 as usize) < 7 && (new_pos.1 as usize) < 7 {
                    // Pawns are weird. Need to check if any opponent piece is in squares
                    // cross ahead. Don't even get me started on "en passant"
                    
                    // First we check for any opposing pieces on the diagonals
                    if vector == &vectors[1] || vector == &vectors[2] {
                        if let Some(p) = &state.0[new_pos.1 as usize][new_pos.0 as usize] {
                            if p.get_color() != self.color {
                                moves.push(new_pos);
                            }
                        }
                    }

                    // Then we check if we can move two pieces forward...
                    else if vector == &vectors[3] {
                        if state.0[new_pos.1 as usize][new_pos.0 as usize].is_none() && self.can_perform {
                            moves.push(new_pos);
                        }
                    }
                    
                    // Lastly, check if we can even move forward
                    else if vector == &vectors[0] {
                        if state.0[new_pos.1 as usize][new_pos.0 as usize].is_none() {
                            moves.push(new_pos);
                        }
                    }
                } else if ChessGame::is_legal_move(state, action) {
                        moves.push(new_pos);
                } else {
                    // If we hit a position that we can't move to, skip to next vector
                    break;
                }

                if ChessGame::contains_opponent_piece(state, &action) {
                    break;
                }
            }
        }

        moves
    }
    
    pub fn get_value(&self) -> f64 {
        self.piece_type.get_value()
    }

    pub fn get_position(&self) -> BoardPosition {
        self.position
    }

    pub fn get_color(&self) -> PlayerColor {
        self.color
    }

    pub fn get_type(&self) -> ChessPieceType {
        self.piece_type
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ChessPieceType {
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

