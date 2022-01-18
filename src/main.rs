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

#[derive(Copy, Clone)]
#[derive(PartialEq)]
enum Marker
{
    X,
    O,
    Empty(usize),
}

impl Marker
{
    fn string(&self) -> String
    {
        match self 
        {
            Marker::X => return String::from("X"),
            Marker::O => return String::from("O"),
            Marker::Empty(size) => return  size.to_string(),
        }
    }
    fn to_color(&self) -> ColoredString
    {
        match self
        {
            Marker::X => return self.string().bold().red(),
            Marker::O => return self.string().bold().blue(),
            Marker::Empty(_) => return self.string().bold(),
        }
    }
    fn opposite(&self) -> Marker
    {
        match self
        {
            Marker::X => return Marker::O,
            Marker::O => return Marker::X,
            Marker::Empty(size) => return Marker::Empty(*size)
        }
    }
    fn win_con(&self) -> State
    {
        match self
        {
            Marker::X => State::WinX,
            Marker::O => State::WinO,
            Marker::Empty(_) => State::Incomplete
        }
    }
    fn random() -> Marker
    {
        let markers : [Marker; 2] = [Marker::O, Marker::X];
        return *markers.choose(&mut rand::thread_rng()).unwrap();
    }
    fn is_empty(&self) -> bool
    {
        match self
        {
            Marker::X | Marker::O => return false,
            Marker::Empty(_) => return true
        }
    }
}


fn user_add(board : &mut[[Marker; 3]; 3], marker : Marker)
{
    loop
    {
        let mut pos = String::new();
        print!("{}: ", marker.to_color());
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

        if board[x][y].is_empty() {
            board[x][y] = marker;
            break
        } else {
            println!("That spot is already taken! Please enter again.");
            continue
        }
    }
}

fn check_win(board : &[[Marker; 3]; 3], marker : Marker) -> bool 
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

fn get_state(board : &[[Marker; 3]; 3]) -> State
{
    if check_win(board, Marker::O) 
    {
        return State::WinO;
    }
    else if check_win(board, Marker::X)
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

fn get_blanks(board : &[[Marker; 3]; 3]) -> Vec<(usize, usize)>
{
    let mut empties = Vec::new();
    for i in 0..3
    {
        for j in 0..3
        {
            if board[i][j].is_empty()
            {
                empties.push((i, j));
            }
        }
    }

    //println!("{:?}", empties);
    empties
}

fn calc_optimal(board : &[[Marker; 3]; 3], marker : Marker) -> ((usize, usize), i32)
{  
    let mut move_list = Vec::new();
    for i in 0..3
    {
        for j in 0..3
        {
            if board[i][j].is_empty()
            {
                let mut mboard = *board;
                mboard[i][j] = marker;
                let state = get_state(&mboard);
                
                if state == marker.win_con()
                {
                    return ((i, j), 1);
                }
                else if state == State::Draw
                {
                    return ((i, j), 0);
                }
                else
                {
                    let score = -1 * calc_optimal(&mboard, marker.opposite()).1; 
                    //Reverse scores for O because no want win
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

fn comp_add(board : &mut[[Marker; 3]; 3], marker : Marker)
{
    println!("{}: Computing...", marker.to_color());
    let spot : (usize,  usize) = calc_optimal(board, marker).0;
    board[spot.0][spot.1] = marker; 
}

fn rand_add(board : &mut[[Marker; 3]; 3], marker : Marker)
{
    let empties = get_blanks(board);
    let pos = empties.choose(&mut rand::thread_rng()).unwrap();
    
    board[pos.0][pos.1] = marker;
}

fn turns(board : &mut[[Marker; 3]; 3], 
    add_x : &dyn Fn(&mut[[Marker; 3]; 3], Marker),
    add_o : &dyn Fn(&mut[[Marker; 3]; 3], Marker)) -> State
{
    let first = Marker::random();
    print_board(board);
    loop
    {
        if first == Marker::X
        {
            if get_state(board) == State::Incomplete { add_x(board, Marker::X); }
            print_board(board);
            if get_state(board) == State::Incomplete { add_o(board, Marker::O); }
            print_board(board);
        }
        if first == Marker::O
        {
            if get_state(board) == State::Incomplete { add_o(board, Marker::O); }
            print_board(board);
            if get_state(board) == State::Incomplete { add_x(board, Marker::X); }
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

fn run_game(board : &mut[[Marker; 3]; 3], 
                add_x : &dyn Fn(&mut[[Marker; 3]; 3], Marker),
                add_o : &dyn Fn(&mut[[Marker; 3]; 3], Marker))
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
            Marker::X.to_color(), 
                        xwins, 
            Marker::O.to_color(), 
                        owins, 
            "Draws".bold(), 
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

fn clear_board(board : &mut[[Marker; 3]; 3])
{
    let mut enumerate = 0;
    for i in (0..3).rev()
    {
        for j in 0..3
        {
            enumerate += 1; 
            board[i][j] = Marker::Empty(enumerate);
        }
    }
}

fn new_board() -> [[Marker; 3]; 3] 
{
    let mut board = [[Marker::Empty(0); 3]; 3];
    clear_board(&mut board);
    return board;
}

fn print_board(board : &[[Marker; 3]; 3])
{
    print!("\x1B[1J\x1B[1;1H");
    let mut enumerate = 0;
    for row in board { 
        for spot in row
        {
            enumerate += 1;
            print!(" {} ", spot.to_color());

            if enumerate % 3 != 0
            {
                print!("{}", "|".bold());
            }
        }

        println!("");
        if enumerate != 9 {println!("{}", format!("---+---+---").bold());}
    }
}

fn choose_player(marker : Marker) -> &'static dyn Fn(&mut[[Marker; 3]; 3], Marker)
{
    loop {
        let mut mstr = String::new();
        print!("Player {} [{}ser/{}PU/{}andom]: ", marker.to_color(), "U".underline(), "C".underline(), "R".underline());
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

    run_game(&mut board, choose_player(Marker::X), choose_player(Marker::O));
}
