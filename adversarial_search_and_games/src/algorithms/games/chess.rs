use std::{
    collections::HashMap,
    thread,
    sync::{Arc,Mutex},
};

use crate::algorithms::game::{Game, Player};


pub type BoardPosition = (usize, usize);
pub type ChessBoard = [[Option<Box<ChessPiece>>; 8]; 8];
/* The chess state consists of the following values:
 * 0. Current board positions
 * 1. Players (and their pieces)
 * 2. Ply
 * 3. History of last 3 states and whether a piece has been taken (to check for three-fold repetition and to reward captures)
 */
pub type ChessState = (ChessBoard, [ChessPlayer; 2], usize, [ChessBoard; 64]);
/* The chess action consists of the following values:
 * 0. Current position of piece
 * 1. New position of piece
 */
pub type ChessAction = (BoardPosition, BoardPosition);

//const BOARD_ROWS:  = 8;
const BOARD_COLS: usize = 8;
const KING_VALUE: f64 = 20.;
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


/* Generic weight matric for positions on the board, need to make one for each piece */
const WEIGHT_MATRIX: [[f64; 8]; 8] = [
    [0.4, 0.6, 1.0, 1.2, 1.2, 1.0, 0.6, 0.4],
    [0.8, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.8],
    [1.0, 1.1, 1.4, 1.6, 1.6, 1.4, 1.1, 1.0],
    [1.1, 1.2, 1.6, 3.0, 3.0, 1.6, 1.2, 1.1],
    [1.1, 1.2, 1.6, 3.0, 3.0, 1.6, 1.2, 1.1],
    [1.0, 1.1, 1.4, 1.6, 1.6, 1.4, 1.1, 1.0],
    [0.8, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.8],
    [0.4, 0.6, 1.0, 1.2, 1.2, 1.0, 0.6, 0.4]
];

pub const OPENERS: [[ChessAction; 8]; 1] = [
    /* Scandinavian */
    [((4,6), (4,4)), ((3,1), (3,3)), // 1: e4 d5
     ((4,4), (3,3)), ((3,0), (3,3)), // 2: d5 Qd5
     ((6,7), (5,5)), ((2,0), (6,4)), // 3: Nf3 Bg4
     ((5,7), (4,6)), ((1,0), (2,2))] // 4: Be2 Nc6
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
    /* Get players */
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

    /* Get information from action */
    fn is_legal_move(state: &ChessState, action: ChessAction) -> bool {
        let cur_pos = action.0;
        let new_pos = action.1;
        if action.1.0  > 7 || action.1.1  > 7 {
            return false;
        }
        let piece = &state.0[cur_pos.1][cur_pos.0];

        let square_value = &state.0[new_pos.1][new_pos.0];
        match square_value {
            Some(other_piece) => {
                // Compare color of piece at destination and color of moving piece
                if piece.is_some() {
                    piece.as_ref().unwrap().get_color() != other_piece.get_color()
                } else {
                    false
                }
            },
            None => {
                true
            }
        }
    }

    fn contains_opponent_piece(state: &ChessState, action: &ChessAction) -> bool {
        let cur_pos = action.0;
        let new_pos = action.1;
        if action.1.0  > 7 || action.1.1  > 7 {
            return false;
        }
        let piece = &state.0[cur_pos.1][cur_pos.0];
        let square_value = &state.0[new_pos.1][new_pos.0];
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

    fn contains_friendly_piece(state: &ChessState, action: &ChessAction) -> bool {
        let cur_pos = action.0;
        let new_pos = action.1;
        if action.1.0  > 7 || action.1.1  > 7 {
            return false;
        }
        let piece = &state.0[cur_pos.1][cur_pos.0];
        let square_value = &state.0[new_pos.1][new_pos.0];
        match square_value {
            Some(other_piece) => {
                // Compare color of piece at destination and color of moving piece
                piece.as_ref().unwrap().get_color() == other_piece.get_color()
            },
            None => {
                false
            }
        }
    }

    /* Moving pieces */
    fn move_piece(&mut self, action: ChessAction) {
        let piece_taken = Self::perform_move(&mut self.board, &action);
        // update players piece
        Self::get_current_player_as_mut(&mut self.board).update_piece(&action);
        if piece_taken.is_some() {
            Self::get_other_player_as_mut(&mut self.board).remove_piece(&action);
        }
        self.board.3[(self.board.2 / 2) % self.board.3.len()] = self.board.0.clone();
        self.board.2 += 1;
    }

    fn hypothetical_move(&self, state: &ChessState, action: &ChessAction) -> ChessState {
        let mut new_state = state.clone();
        
        let piece_taken = Self::perform_move(&mut new_state, &action);
        // update players piece
        if let Some(promoted_pawn) = Self::get_current_player_as_mut(&mut new_state).update_piece(&action) {
            let pos = promoted_pawn.get_position().clone();
            new_state.0[pos.1][pos.0] = Some(Box::new(promoted_pawn));
        }
        if piece_taken.is_some() {
            Self::get_other_player_as_mut(&mut new_state).remove_piece(&action);
        }
        let state_copy = new_state.clone();
        Self::get_other_player_as_mut(&mut new_state).check_if_checked(&state_copy, Self::get_current_player(&state_copy));
        Self::get_current_player_as_mut(&mut new_state).last_move = Some(*action);
   
        // Log state in history buffer
        new_state.3[(state.2 / 2) % state.3.len()] = new_state.0.clone();

        new_state.2 += 1;
        new_state
    }

    fn perform_move(state: &mut ChessState, action: &ChessAction) -> Option<ChessPieceType> {
        let cur_pos = action.0;
        let new_pos = action.1;
        let piece_taken = if state.0[new_pos.1][new_pos.0].is_some() { Some(state.0[new_pos.1][new_pos.0].as_ref().unwrap().get_type()) } else { None };
        let mut piece = state.0[cur_pos.1][cur_pos.0].as_mut().unwrap();
        // update piece's own state
        piece.position = action.1;

        state.0[new_pos.1][new_pos.0] = Some(piece.clone());
        state.0[cur_pos.1][cur_pos.0] = None;
        
        piece_taken
    }

    /* Functions for evaluating current state */
    fn get_total_piece_value(state: &ChessState, player: &ChessPlayer) -> f64 {
        let other_player = Self::get_other_player(state);
        let k = if player.king.is_some() { 1. } else { 0. };
        let k_m = if other_player.king.is_some() { 1. } else { 0. };
        200. * (k - k_m)
        + 9. * (player.queens.len() as f64 - other_player.queens.len() as f64)
        + 5. * (player.rooks.len() as f64 - other_player.rooks.len() as f64)
        + 3. * (player.knights.len() as f64 - other_player.knights.len() as f64 + player.bishops.len() as f64 - other_player.bishops.len() as f64)
        + 1. * (player.pawns.len() as f64 - other_player.pawns.len() as f64)
    }

    fn get_weighted_attacked_pieces_value(state: &ChessState, player: &ChessPlayer) -> f64 {
        let attacked_pieces = player.get_attacked_pieces(state);
        let mut total_value: f64 = 0.;
        for piece in &attacked_pieces {
            total_value += piece.1.get_value() * WEIGHT_MATRIX[piece.0.1][piece.0.0];
        }
        total_value * 0.1
    }

    fn get_weighted_defended_pieces_value(state: &ChessState, player: &ChessPlayer) -> f64 {
        let defended_pieces = player.get_defended_pieces(state);
        let mut total_value: f64 = 0.;
        for piece in &defended_pieces {
            total_value += piece.1.get_value() * WEIGHT_MATRIX[piece.0.1][piece.0.0]; 
        }

        total_value * 0.1
    }

    fn get_weighted_available_moves(state: &ChessState, player: &ChessPlayer) -> f64 {
        let moves = player.get_moves(state);
        let mut total: f64 = 0.;

        for m in &moves {
            total += WEIGHT_MATRIX[m.1.1][m.1.0];
        }
        0.1 * (total + moves.len() as f64)
    }

    fn get_repetition_penalty(state: &ChessState) -> f64 {
        let mut penalty: f64 = 0.;
        for i in 2..state.3.len() {
            if state.3[i] == state.3[i - 2] {
                penalty += 500.;
            }
        }
        penalty
    }
}

impl Game<ChessState, ChessAction, ChessPlayer> for ChessGame {
    fn create_game() -> Self {
        let mut new_game = Self {
            board: (array![array![None; 8]; 8], [ChessPlayer::new(PlayerColor::White),
                      ChessPlayer::new(PlayerColor::Black)], 0, array![array![array![None; 8]; 8]; 64]),
        };

        for p in &new_game.board.1 {
            // place king
            new_game.board.0[p.king.as_ref().unwrap().get_position().1 ][p.king.as_ref().unwrap().get_position().0 ] = p.king.clone();
        
            // place queen(s?)
            for q in &p.queens {
                let pos = q.get_position();
                new_game.board.0[pos.1][pos.0] = Some(q.clone());
            }

            // place rooks
            for r in &p.rooks {
                let pos = r.get_position();
                new_game.board.0[pos.1][pos.0] = Some(r.clone());
            }

            // place knights
            for n in &p.knights {
                let pos = n.get_position();
                new_game.board.0[pos.1][pos.0] = Some(n.clone());
            }

            // place bishops
            for b in &p.bishops {
                let pos = b.get_position();
                new_game.board.0[pos.1][pos.0] = Some(b.clone());
            }

            // place pawns
            for pawn in &p.pawns {
                let pos = pawn.get_position();
                new_game.board.0[pos.1][pos.0] = Some(pawn.clone());
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
        if Self::get_current_player(state).king == None || Self::get_current_player(state).get_moves(state).len() == 0 || (state.3[0] == state.3[1] && state.3[1] == state.3[2]) {
            true
        } else {
            false
        }
    }

    fn utility(&self, state: &ChessState, player: &ChessPlayer) -> f64 {
        let other_player = Arc::new(Mutex::new(Self::get_other_player(state).clone()));
        let player = Arc::new(Mutex::new(player.clone()));
        let state = Arc::new(Mutex::new(state.clone()));
        let t_state = Arc::clone(&state);
        let t_player = Arc::clone(&player);
        let a_state = Arc::clone(&state);
        let a_player = Arc::clone(&player);
        let a_other = Arc::clone(&other_player);
        let d_state = Arc::clone(&state);
        let d_player = Arc::clone(&player);
        let d_other = Arc::clone(&other_player);
        let m_state = Arc::clone(&state);
        let m_player = Arc::clone(&player);
        let m_other = Arc::clone(&other_player);
        let p_state = Arc::clone(&state);
        thread::spawn(move || {
            Self::get_total_piece_value(&t_state.lock().unwrap(), &t_player.lock().unwrap())
        }).join().unwrap()
        + 
        thread::spawn(move || {
            let a_player = a_player.lock().unwrap();
            let a_state = a_state.lock().unwrap();
            let a_other = a_other.lock().unwrap();
            Self::get_weighted_attacked_pieces_value(&a_state, &a_player) - Self::get_weighted_attacked_pieces_value(&a_state, &a_other)
        }).join().unwrap()
        + 
        thread::spawn(move || {
            let d_player = d_player.lock().unwrap();
            let d_state = d_state.lock().unwrap();
            let d_other = d_other.lock().unwrap();
            Self::get_weighted_defended_pieces_value(&d_state, &d_player) - Self::get_weighted_attacked_pieces_value(&d_state, &d_other)
        }).join().unwrap()
        + 
        thread::spawn(move || {
            let m_player = m_player.lock().unwrap();
            let m_state = m_state.lock().unwrap();
            let m_other = m_other.lock().unwrap();
            Self::get_weighted_available_moves(&m_state, &m_player) - Self::get_weighted_available_moves(&m_state, &m_other)
        }).join().unwrap()
        -
        thread::spawn(move || {
            let p_state = p_state.lock().unwrap();
            Self::get_repetition_penalty(&p_state)
        }).join().unwrap()
    }

    fn take_action(&mut self, _state: &ChessState, action: &ChessAction) -> &ChessState {
        self.move_piece(action.clone());
        &self.board
    }
}

#[derive(Clone, Debug)]
pub struct ChessPlayer {
    color: PlayerColor,
    pieces: HashMap<(usize, usize), (ChessPieceType, usize)>,
    king: Option<Box<ChessPiece>>,
    queens: Vec<Box<ChessPiece>>,
    rooks: Vec<Box<ChessPiece>>,
    knights: Vec<Box<ChessPiece>>,
    bishops: Vec<Box<ChessPiece>>,
    pawns: Vec<Box<ChessPiece>>,
    checked_by: Option<ChessPiece>,
    last_move: Option<ChessAction>
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
            checked_by: None,
            last_move: None,
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
        if self.king.is_none() {
            return Vec::new();
        }
        let mut all_moves: Vec<ChessAction> = Vec::new();
        let mut checked_squares: Vec<BoardPosition> = Vec::new();
        let mut other_players_moves = Vec::new();
        let other_player = ChessGame::get_other_player(state);
        if self.color == ChessGame::get_current_player(state).color {
            other_players_moves.append(&mut other_player.get_moves(state));
        }
        if self.checked_by.is_some() {
            let other_pos = self.checked_by.as_ref().unwrap().get_position();
            let king_pos = self.king.as_ref().unwrap().get_position();
            let max_length = (other_pos.0 as isize - king_pos.0 as isize).max(other_pos.1 as isize - king_pos.1 as isize);
            let diff = (other_pos.0 as isize - king_pos.0 as isize, other_pos.1 as isize - king_pos.1 as isize);
            match self.checked_by.as_ref().unwrap().get_type() {
                ChessPieceType::Knight => {
                    /* We handle knights differently, as they aren't stopped by LOS */
                    checked_squares.push(other_pos);
                },
                _ => {
                    /* All pieces but knights */
                    for i in 1..=max_length {
                        let offset = ((diff.0 / max_length) * i, (diff.1 / max_length) * i);
                        let new_pos = (king_pos.0 as isize + offset.0, king_pos.1 as isize + offset.0);
                        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                        checked_squares.push(new_pos);
                    }
                }
            }
        }
        
        for m in self.king.as_ref().unwrap().get_possible_moves(state) {
            if !checked_squares.contains(&m) && !other_player.attacked_squares(&other_players_moves).contains(&m) {
                all_moves.push((self.king.as_ref().unwrap().get_position(), m)); // If the move puts us outside of checked line
            }
        }
        for queen in &self.queens {
            for m in queen.get_possible_moves(state) {
                if self.checked_by.is_none() || checked_squares.contains(&m) {
                    /*if state.0[m.1][m.0].is_some() {
                        is_check = state.0[m.1][m.0].as_ref().unwrap().get_type() == ChessPieceType::King;
                    }*/
                    all_moves.push((queen.get_position(), m));
                }
            }
        }
        for rook in &self.rooks {
            for m in rook.get_possible_moves(state) {
                if self.checked_by.is_none() || checked_squares.contains(&m) {
                    /*if state.0[m.1][m.0].is_some() {
                        is_check = state.0[m.1][m.0].as_ref().unwrap().get_type() == ChessPieceType::King;
                    }*/
                    all_moves.push((rook.get_position(), m));
                }
            }
        }
        for knight in &self.knights {
            for m in knight.get_possible_moves(state) {
                if self.checked_by.is_none() || checked_squares.contains(&m) {
                    /*if state.0[m.1][m.0].is_some() {
                        is_check = state.0[m.1][m.0].as_ref().unwrap().get_type() == ChessPieceType::King;
                    }*/
                    all_moves.push((knight.get_position(), m));
                }
            }
        }
        for bishop in &self.bishops {
            for m in bishop.get_possible_moves(state) {
                if self.checked_by.is_none() || checked_squares.contains(&m) {
                    /*if state.0[m.1][m.0].is_some() {
                        is_check = state.0[m.1][m.0].as_ref().unwrap().get_type() == ChessPieceType::King;
                    }*/
                    all_moves.push((bishop.get_position(), m));
                }
            }
        }
        for pawn in &self.pawns {
            for m in pawn.get_possible_moves(state) {
                if self.checked_by.is_none() || checked_squares.contains(&m) {
                    /*if state.0[m.1][m.0].is_some() {
                        is_check = state.0[m.1][m.0].as_ref().unwrap().get_type() == ChessPieceType::King;
                    }*/
                    all_moves.push((pawn.get_position(), m));
                }
            }
        }

        all_moves
    }

    pub fn get_color(&self) -> PlayerColor {
        self.color
    }

    fn update_piece(&mut self, action: &ChessAction) -> Option<ChessPiece> {
        let piece_info = self.pieces.get(&action.0).clone();
        let mut promoted_pawn = None;
        if piece_info.is_some() {
            let mut piece_info = *piece_info.unwrap();
            match piece_info.0 {
                ChessPieceType::King => {
                    self.king.as_mut().unwrap().position = action.1;
                    self.king.as_mut().unwrap().can_perform = false;
                },
                ChessPieceType::Queen => { 
                    if piece_info.1 > self.queens.len() {
                        println!("Queens: {:?}", self.queens);
                        println!("Piece_info: {:?}", piece_info);
                        println!("Pieces: {:?}", self.pieces);
                        loop {}
                    }
                    self.queens[piece_info.1].position = action.1;
                },
                ChessPieceType::Rook => {
                    self.rooks[piece_info.1].position = action.1;
                    self.rooks[piece_info.1].can_perform = false;
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
                    match self.color {
                        PlayerColor::White => {
                            if action.1.1 == 0 {
                                /*
                                let mut pawn = self.pawns[piece_info.1].clone();
                                self.remove_piece(action);
                                pawn.piece_type = ChessPieceType::Queen;
                                self.queens.push(pawn);
                                piece_info.0 = ChessPieceType::Queen;
                                piece_info.1 = self.queens.len() - 1;
                                */
                                promoted_pawn = Some(self.promote_pawn(action).clone());
                                piece_info.0 = ChessPieceType::Queen;
                                piece_info.1 = self.queens.len() - 1;
                            }
                        },
                        PlayerColor::Black => {
                            if action.1.1 == 7 {
                                /*
                                let mut pawn = self.pawns[piece_info.1].clone();
                                self.remove_piece(action);
                                pawn.piece_type = ChessPieceType::Queen;
                                self.queens.push(pawn);
                                piece_info.0 = ChessPieceType::Queen;
                                piece_info.1 = self.queens.len() - 1;
                                */
                                promoted_pawn = Some(self.promote_pawn(action).clone());
                                piece_info.0 = ChessPieceType::Queen;
                                piece_info.1 = self.queens.len() - 1;
                            }
                        }
                    }
                }
            }
            self.pieces.remove(&action.0);
            self.pieces.insert(action.1, piece_info);
        }

        promoted_pawn
    }

    fn remove_piece(&mut self, action: &ChessAction) {
        let piece_info = self.pieces.get(&action.1).clone();
        if piece_info.is_some() {
            let piece_info = *piece_info.unwrap();
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
        
            for p in self.pieces.values_mut() {
                if p.0 == piece_info.0 && p.1 > piece_info.1 {
                    p.1 -= 1;
                }
            }

            self.pieces.remove(&action.1);
        }
    }

    fn get_attacked_pieces(&self, state: &ChessState) -> Vec<(BoardPosition, ChessPieceType)> {    
        let mut attacked_pieces = Vec::new();
        if self.king.is_none() {
            return Vec::new();
        }
        attacked_pieces.append(&mut self.king.as_ref().unwrap().get_attacks(state));
        for queen in &self.queens {
            attacked_pieces.append(&mut queen.get_attacks(state));
        }
        for rook in &self.rooks {
            attacked_pieces.append(&mut rook.get_attacks(state));
        }
        for knight in &self.knights {
            attacked_pieces.append(&mut knight.get_attacks(state));
        }
        for bishop in &self.bishops {
            attacked_pieces.append(&mut bishop.get_attacks(state));
        }
        for pawn in &self.pawns {
            attacked_pieces.append(&mut pawn.get_attacks(state));
        }

        attacked_pieces
    }

    fn get_defended_pieces(&self, state: &ChessState) -> Vec<(BoardPosition, ChessPieceType)> {
        let mut defended_pieces = Vec::new();
        if self.king.is_none() {
            return Vec::new();
        }
        defended_pieces.append(&mut self.king.as_ref().unwrap().get_defends(state));
        for queen in &self.queens {
            defended_pieces.append(&mut queen.get_defends(state));
        }
        for rook in &self.rooks {
            defended_pieces.append(&mut rook.get_defends(state));
        }
        for knight in &self.knights {
            defended_pieces.append(&mut knight.get_defends(state));
        }
        for bishop in &self.bishops {
            defended_pieces.append(&mut bishop.get_defends(state));
        }
        for pawn in &self.pawns {
            defended_pieces.append(&mut pawn.get_defends(state));
        }

        defended_pieces
    }

    fn attacked_squares(&self, moves: &Vec<ChessAction>) -> Vec<BoardPosition> {
        let mut attacked_squares = Vec::new();
        for m in moves {
            attacked_squares.push(m.1);  
        }
        attacked_squares
    }

    fn check_if_checked(&mut self, state: &ChessState, other_player: &Self) -> bool {
        if self.king.is_none() {
            return true;
        }
        let mut is_checked = false;
        self.checked_by = None;
        for m in other_player.get_moves(state) {
            if m.1 == self.king.as_ref().unwrap().get_position() {
                is_checked = true;
                self.checked_by = Some(*state.0[m.0.1][m.0.0].clone().unwrap());
            }
        }

        is_checked
    }

    fn promote_pawn(&mut self, action: &ChessAction) -> &ChessPiece {
        let piece_info = self.pieces.get(&action.0).unwrap();
        let mut pawn_to_queen = self.pawns.get(piece_info.1).unwrap().clone();
        self.remove_piece(action);
        pawn_to_queen.piece_type = ChessPieceType::Queen;
        self.queens.push(pawn_to_queen.clone());
        self.queens.last().unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChessPiece {
    piece_type: ChessPieceType,
    position: BoardPosition,
    color: PlayerColor,
    can_perform: bool, // this is used for initial movement of pawns and castling of kings
}

impl ChessPiece {
    fn get_possible_moves(&self, state: &ChessState) -> Vec<BoardPosition> {
        let mut moves: Vec<BoardPosition> = Vec::new();
        let vectors: Vec<(isize, isize)>;
        let range: usize;
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
                    new_pos = ((new_pos.0 as isize + vector.0) as usize, (new_pos.1 as isize - vector.1) as usize);
                } else {
                    new_pos = ((new_pos.0 as isize + vector.0) as usize, (new_pos.1 as isize + vector.1) as usize);
                }
                let action: ChessAction = (self.position, new_pos);
                
                if self.piece_type == ChessPieceType::Pawn && (new_pos.0 ) < 7 && (new_pos.1 ) < 7 {
                    // Pawns are weird. Need to check if any opponent piece is in squares
                    // cross ahead. Don't even get me started on "en passant"

                    // First we check for any opposing pieces on the diagonals
                    if vector == &vectors[1] || vector == &vectors[2] {
                        if let Some(p) = &state.0[new_pos.1 ][new_pos.0 ] {
                            if p.get_color() != self.color {
                                moves.push(new_pos);
                            }
                        }
                    }

                    // Lastly, check if we can even move forward
                    else if vector == &vectors[0] {
                        if state.0[new_pos.1][new_pos.0].is_none() {
                            moves.push(new_pos);

                            /* Check if we can move two squares ahead */
                            if self.can_perform {
                                if self.color == PlayerColor::White {
                                    new_pos = (self.position.0, self.position.1 - vectors[3].1 as usize);
                                } else {
                                    new_pos = (self.position.0, self.position.1 + vectors[3].1 as usize);
                                }
                            
                                if state.0[new_pos.1][new_pos.0].is_none() {
                                    moves.push(new_pos);
                                }
                            }
                        }
                    }
                } else if ChessGame::is_legal_move(state, action) {
                    moves.push(new_pos);
                    if ChessGame::contains_opponent_piece(state, &action) {
                        break;
                    }
                } else {
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

    pub fn get_attacks(&self, state: &ChessState) -> Vec<(BoardPosition, ChessPieceType)> {    
        let mut attacks = Vec::new();
        for m in self.get_possible_moves(state) {
            let a = (self.position, m);
            if ChessGame::contains_opponent_piece(state, &a) {
                attacks.push((m, state.0[m.1][m.0].as_ref().unwrap().get_type()));
            }
        }

        attacks
    }

    pub fn get_defends(&self, state: &ChessState) -> Vec<(BoardPosition, ChessPieceType)> {
        let mut attacks = Vec::new();
        for m in self.get_possible_moves(state) {
            let a = (self.position, m);
            if ChessGame::contains_friendly_piece(state, &a) {
                attacks.push((m, state.0[m.1][m.0].as_ref().unwrap().get_type()));
            }
        }

        attacks
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
