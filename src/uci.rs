use std::io;

mod uci {
    fn parse_command(command: String){
        match command.as_str() {
            "uci" => println!("id name rustybunny\nuciok\n"),
            "isready" => println!("readyok\n"),
            "ucinewgame" => (),
            "position" => (),
            "go" => (),
            "stop" => (),
            _ => ()
        }
    }
}



// parser copied from kockasfulu mk2

// #include "uci.hpp"

// #include <chess.hpp>
// #include <iostream>
// #include "globals.hpp"
// #include "utils.hpp"
// #include "search.hpp"

// namespace
// {
//     void parse_moves(std::istringstream &iss)
//     {
//         std::string moves;
//         iss >> moves;
//         std::string move;
//         while (iss >> move)
//         {
//             current_board.makeMove(chess::uci::uciToMove(current_board, move));
//         }
//     }

//     void position(std::istringstream &iss)
//     {
//         std::string fen_or_startpos;
//         iss >> fen_or_startpos;
//         if (fen_or_startpos == "fen")
//         {
//             std::string fenstring;
//             for (int i = 0; i < 5; i++)
//             {
//                 iss >> fenstring;
//                 fenstring.append(" ");
//             }
//             iss >> fenstring;

//             current_board = chess::Board{fenstring};
//             parse_moves(iss);
//         }
//         else if (fen_or_startpos == "startpos")
//         {
//             current_board = chess::Board{STARTER_FEN};
//             parse_moves(iss);
//         }
//     }

//     void go(std::istringstream & iss){
//         std::string dummy;
//         int wtime;
//         int btime;
//         int winc;
//         int binc;
//         iss >> dummy >> wtime >> dummy >> btime >> dummy >> winc >> dummy >> binc;

//         auto bestmove = findBestMove(current_board, wtime, btime, winc, binc);

//         std::cout << "info score cp " << bestmove.score << "\n";
//         std::cout << "bestmove " << chess::uci::moveToUci(bestmove.move) << "\n";
//     }
// }

// void parse_command(std::string &line)
// {
//     std::istringstream iss{line};
//     std::string command;
//     iss >> command;
//     if (command == "uci")
//     {
//         // @TODO: separate function
//         std::cout << "id name Kockasfulu-mk2\n"; // @TODO: add options
//         std::cout
//             << "uciok\n";
//     }
//     if (command == "isready")
//     {
//         std::cout << "readyok\n";
//     }
//     if (command == "ucinewgame")
//     {
//         current_board = chess::Board{STARTER_FEN};
//     }
//     if (command == "position")
//     {
//         position(iss);
//     }
//     if (command == "go"){
//         go(iss);
//     }
//     if (command == "stop"){
//         exit(0);
//     }
// }
