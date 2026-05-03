use chess::{Board, BoardStatus, ChessMove, Color, MoveGen, Piece};

pub fn calculate(board: &mut Board, depth: isize) -> ChessMove {

    

    let mut iterable = MoveGen::new_legal(&board);
    let mut best_move = ChessMove::default();
    let mut best_value = isize::MIN + 1;
    let mut alpha = isize::MIN + 1;
    let beta = isize::MAX;

    for chessmove in &mut iterable {
        let mut new_board = board.make_move_new(chessmove); // How is this not slow? no unmake? what?
        let new_board_ref: &mut Board = &mut new_board;
        let score = -negamax(-beta, -alpha, depth - 1, new_board_ref);
        if score > best_value {
            best_value = score;
            best_move = chessmove;
            if score > alpha {
                alpha = score;
            }
        }
        if score >= beta {
            break;
        }
    }
    return best_move;
}

fn negamax(mut alpha: isize, beta: isize, depthleft: isize, board: &mut Board) -> isize {

    if board.status() == BoardStatus::Checkmate {
        return isize::MIN + 1;
    }
    //    if  depthleft == 0 {return quiesce( alpha, beta );}
    if depthleft == 0 {
        if board.side_to_move() == Color::White {
            return evaluate(board);
        } else {
            return -evaluate(board);
        }
    }
    let mut best_value = isize::MIN + 1;

    let mut iterable = MoveGen::new_legal(&board);

    for chessmove in &mut iterable {
        let mut new_board = board.make_move_new(chessmove); // How is this not slow? no unmake? what?
        let new_board_ref: &mut Board = &mut new_board;
        let score = -negamax(-beta, -alpha, depthleft - 1, new_board_ref);
        if score > best_value {
            best_value = score;
            if score > alpha {
                alpha = score;
            }
        }
        if score >= beta {
            break;
        }
    }
    return best_value;
}

fn evaluate(board: &mut Board) -> isize {
    let mut score: isize = 0;

    score +=
        ((*board.pieces(Piece::Pawn) & *board.color_combined(Color::White)).count() * 100) as isize;
    score += ((*board.pieces(Piece::Bishop) & *board.color_combined(Color::White)).count() * 300)
        as isize;
    score += ((*board.pieces(Piece::Knight) & *board.color_combined(Color::White)).count() * 300)
        as isize;
    score +=
        ((*board.pieces(Piece::Rook) & *board.color_combined(Color::White)).count() * 500) as isize;
    score += ((*board.pieces(Piece::Queen) & *board.color_combined(Color::White)).count() * 900)
        as isize;

    score -=
        ((*board.pieces(Piece::Pawn) & *board.color_combined(Color::Black)).count() * 100) as isize;
    score -= ((*board.pieces(Piece::Bishop) & *board.color_combined(Color::Black)).count() * 300)
        as isize;
    score -= ((*board.pieces(Piece::Knight) & *board.color_combined(Color::Black)).count() * 300)
        as isize;
    score -=
        ((*board.pieces(Piece::Rook) & *board.color_combined(Color::Black)).count() * 500) as isize;
    score -= ((*board.pieces(Piece::Queen) & *board.color_combined(Color::Black)).count() * 900)
        as isize;

    return score;
}

// TODO: negamax framework

// int alphaBeta( int alpha, int beta, int depthleft ) {
//    if( depthleft == 0 ) return quiesce( alpha, beta );
//    bestValue = -infinity;
//    for ( all moves)  {
//       score = -alphaBeta( -beta, -alpha, depthleft - 1 );
//       if( score > bestValue )
//       {
//          bestValue = score;
//          if( score > alpha )
//             alpha = score; // alpha acts like max in MiniMax
//       }
//       if( score >= beta )
//          return bestValue;   //  fail soft beta-cutoff, existing the loop here is also fine
//    }
//    return bestValue;
// }

// TODO: transposition table

// TODO: move ordering - hash moves, principal variation

// TODO: Iterative deepening - stop command - time management

// TODO: aspiration window

// TODO: quiescence search

// int Quiesce( int alpha, int beta ) {
//     int static_eval = Evaluate();

//     // Stand Pat
//     int best_value = static_eval;
//     if( best_value >= beta )
//         return best_value;
//     if( best_value > alpha )
//         alpha = best_value;

//     until( every_capture_has_been_examined )  {
//         MakeCapture();
//         score = -Quiesce( -beta, -alpha );
//         TakeBackMove();

//         if( score >= beta )
//             return score;
//         if( score > best_value )
//             best_value = score;
//         if( score > alpha )
//             alpha = score;
//     }

//     return best_value;
// }
