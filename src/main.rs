//  xo-rs: A terminal tic tac toe game
//  Copyright (C) 2022  Collin Francel
//  
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//  
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//  
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// If you have any questions, contact me at collin.t.francel@gmail.com      

use std::vec::Vec;
use rand::seq::SliceRandom;
//use rand::Rng;
use std::io::{self, Write};
use colored::*;

#[derive(PartialEq)]
enum State
{
    WinO,
    WinX,
    Incomplete,
    Draw,
}

fn user_add<'a>(board : &mut[[&'a str; 3]; 3], marker : &'a str)
{
    loop
    {
        let mut pos = String::new();
        print!("{}: ", to_color(marker));
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut pos)
            .expect("Failed to read line");
    
        let num : usize = match pos.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to read number. Please enter again.");
                continue
            },
        };
        if !(num >= 1 && num <= 9)
        {
            println!("That is not a choice! Please enter again.");
            continue
        }

        let x = 2 - ((num-1) - (num-1) % 3) / 3;
        let y = (num+2) % 3; 

        if board[x][y] != "X" &&  board[x][y] != "O" {
            board[x][y] = marker;
            break
        } else {
            println!("That spot is already taken! Please enter again.");
            continue
        }
    }
}

fn check_win(board : &[[&str; 3]; 3], marker : &str) -> bool 
{
    let mut winul : bool = board[0][0] == marker;
    let mut winur : bool = board[2][0] == marker;
    for i in 0..3
    {
        let mut winrow : bool = board[i][0] == marker;
        let mut wincol : bool = board[0][i] == marker;

        for j in 0..3
        {
            winrow = winrow && board[i][j] == marker;
            wincol = wincol && board[j][i] == marker;
            if i == j
            {
                winul = winul && board[i][j] == marker;
            } 
            if (2 - i) == j {
                winur = winur && board[i][j] == marker;
            }
        }
        if winrow || wincol 
        {
            return winrow || wincol;
            //println!("wincol: {}", wincol);
            //println!("winrow: {}", winrow);
        }
    }

    //println!("winur: {}", winur);
    //println!("winul: {}", winul);

    winul || winur
}

fn get_state(board : &[[&str; 3]; 3]) -> State
{
    if check_win(board, "O") 
    {
        return State::WinO;
    }
    else if check_win(board, "X")
    {
        return State::WinX;
    }

    if get_blanks(board).len() == 0
    {
        return State::Draw;
    }
    else
    {
        return State::Incomplete;
    }
}

fn opp(marker : &str) -> &str
{
    if marker == "X"
    {
        "O"
    }
    else if marker == "O"
    {
        "X"
    }
    else
    {
        " "
    }
}

fn win_con(marker : &str) -> State
{
    if marker == "X"
    {
        return State::WinX;
    }
    else if marker == "O"
    {
        return State::WinO;
    }
    else
    {
        return State::Incomplete;
    }
}

fn get_blanks(board : &[[&str; 3]; 3]) -> Vec<(usize, usize)>
{
    let mut empties = Vec::new();
    for i in 0..3
    {
        for j in 0..3
        {
            if board[i][j] != "O" && board[i][j] != "X"
            {
                empties.push((i, j));
            }
        }
    }

    //println!("{:?}", empties);
    empties
}

fn calc_optimal<'a>(board : &[[&'a str; 3]; 3], marker : &'a str) -> ((usize, usize), i32)
{  
    let mut move_list = Vec::new();
    for i in 0..3
    {
        for j in 0..3
        {
            if board[i][j] != "O" && board[i][j] != "X"
            {
                let mut mboard = *board;
                mboard[i][j] = marker;
                let state = get_state(&mboard);
                
                if state == win_con(marker) //Note: shouldn't have to handle win_con(opp(marker))
                {
                    return ((i, j), 1); //Score of 1 for winning game
                }
                else if state == State::Draw
                {
                    return ((i, j), 0); //Board filled up, no more left to do
                }
                else
                {
                    let score = -1 * calc_optimal(&mboard, opp(marker)).1; //Reverse scores for O because no want win
                    move_list.push(((i, j), score));
                }
            }
        }
    }

    move_list.sort_by(|a, b| a.1.cmp(&b.1)); //Order it I think
    //println!("{:?}", move_list);
    let len = move_list.len();
    let mut bottom_index = len - 1;
    for (i, entry) in move_list.iter().enumerate().rev()
    {
        if move_list[bottom_index].1 == entry.1
        {
            bottom_index = i;
        }
        else
        {
            break;
        }
    }

    //if rand::thread_rng().gen_bool(7.0 / 100.0) && bottom_index != 0
    //{
    //    bottom_index -= 1;
    //    println!("he messd up :joy:");
    //} //10% * 1 / (top_list.len() + 1) = chance of suboptimal play

    let top_list : Vec<((usize,usize), i32)> = move_list[bottom_index..].to_vec();
    
    *top_list.choose(&mut rand::thread_rng()).unwrap() //Return one with the top score
}

fn comp_add<'a>(board : &mut[[&'a str; 3]; 3], marker : &'a str)
{
    println!("{}: Computing...", to_color(marker));
    let spot : (usize,  usize) = calc_optimal(board, marker).0;
    board[spot.0][spot.1] = marker; 
}

fn rand_add<'a>(board : &mut[[&'a str; 3]; 3], marker : &'a str)
{
    let empties = get_blanks(board);
    let pos = empties.choose(&mut rand::thread_rng()).unwrap();
    
    board[pos.0][pos.1] = marker;
}

fn turns<'a>(board : &mut[[&'a str; 3]; 3], 
    add_x : &dyn Fn(&mut[[&'a str; 3]; 3], &'a str),
    add_o : &dyn Fn(&mut[[&'a str; 3]; 3], &'a str)) -> State
{
    let markers : [&'a str; 2] = ["O","X"];
    let first = markers.choose(&mut rand::thread_rng()).unwrap(); //Choose randomly between markers
    print_board(board);
    loop
    {
        if *first == "X"
        {
            if get_state(board) == State::Incomplete { add_x(board, "X"); }
            print_board(board);
            if get_state(board) == State::Incomplete { add_o(board, "O"); }
            print_board(board);
        }
        if *first == "O"
        {
            if get_state(board) == State::Incomplete { add_o(board, "O"); }
            print_board(board);
            if get_state(board) == State::Incomplete { add_x(board, "X"); }
            print_board(board);
        }
        let state = get_state(board);
        if state != State::Incomplete 
        { 
            
            clear_board(board);
            return state; 
        }     
    }
}

fn run_game<'a>(board : &mut[[&'a str; 3]; 3], 
                add_x : &dyn Fn(&mut[[&'a str; 3]; 3], &'a str),
                add_o : &dyn Fn(&mut[[&'a str; 3]; 3], &'a str))
{ //Complicated function reference moment
    let mut xwins = 0;
    let mut owins = 0;
    let mut draws = 0;
    'game: loop
    {
        match turns(board, add_x, add_o)
        {
            State::WinO => owins += 1,
            State::WinX => xwins += 1,
            State::Draw => draws += 1,
            _ => println!("Bruh")
        }
        println!("{}: {}, {}: {}, {}: {}", 
            to_color("X"), 
                        xwins, 
            to_color("O"), 
                        owins, 
            to_color("Draws"), 
                        draws);
        
        loop
        {
            let mut response = String::new();
            print!("Play again? [Y/n] ");
            let _ = io::stdout().flush();
            io::stdin()
                .read_line(&mut response)
                .expect("Failed to read line");
        
            let response = response.trim();

            match response
            {
                "Y" | "y" | "" => break,
                "N" | "n" => break 'game,
                _ => println!("Failed to read input. Please try again."),
            }
        }
    }
}

fn clear_board(board : &mut[[& str; 3]; 3])
{
    *board = [["7", "8", "9"], 
             ["4", "5", "6"], 
             ["1", "2", "3"]];
}

fn new_board<'a>() -> [[&'a str; 3]; 3] 
{
    let mut board = [[" "; 3]; 3];
    clear_board(&mut board);
    return board;
}

fn to_color(text : &str) -> colored::ColoredString
{
    match text {
        "X" => return text.red().bold(),
        "O" => return text.blue().bold(),
        _ => return text.bold()
    }
}

fn print_board(board : &[[&str; 3]; 3])
{
    print!("\x1B[1J\x1B[1;1H");
    let mut enumerate = 0;
    for row in board { 
        for spot in row
        {
            enumerate += 1;
            print!(" {} ", to_color(spot));

            if enumerate % 3 != 0
            {
                print!("{}", "|".bold());
            }
        }

        println!("");
        if enumerate != 9 {println!("{}", format!("---+---+---").bold());}
    }
}

fn choose_player<'a>(marker : &str) -> &dyn Fn(&mut[[&'a str; 3]; 3], &'a str)
{
    loop {
        let mut mstr = String::new();
        print!("Player {} [{}ser/{}PU/{}andom]: ", to_color(marker), "U".underline(), "C".underline(), "R".underline());
        let _ = io::stdout().flush();
        io::stdin()
                .read_line(&mut mstr)
                .expect("Failed to read line");

        match mstr.as_str().trim().chars().nth(0).unwrap().to_string().as_str() {
            //This is awful lmao
            //String to &str, remove spaces, convert to char list, 
            //get first element, unwrap to u8, convert to String, convert to &str
            "U" | "u" => return &user_add,
            "C" | "c" => return &comp_add,
            "R" | "r" => return &rand_add,
            _ => {
                println!("Input could not be read. Please try again.");
                continue;
            }
        }
    }
}

fn main() {
    #[cfg(windows)]
    {
        control::set_virtual_terminal(true).unwrap();
    }

    let mut board = new_board();

    run_game(&mut board, choose_player("X"), choose_player("O"));
}
