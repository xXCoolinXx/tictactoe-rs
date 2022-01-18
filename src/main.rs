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

struct Board
{
    array : [[Marker; 3]; 3]
}

impl Board {
    fn user_add(&mut self, marker : Marker)
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

            if self.array[x][y].is_empty() {
                self.array[x][y] = marker;
                break
            } else {
                println!("That spot is already taken! Please enter again.");
                continue
            }
        }
    }

    fn check_win(&self, marker : Marker) -> bool 
    {
        let mut winul : bool = self.array[0][0] == marker;
        let mut winur : bool = self.array[2][0] == marker;
        for i in 0..3
        {
            let mut winrow : bool = self.array[i][0] == marker;
            let mut wincol : bool = self.array[0][i] == marker;

            for j in 0..3
            {
                winrow = winrow && self.array[i][j] == marker;
                wincol = wincol && self.array[j][i] == marker;
                if i == j
                {
                    winul = winul && self.array[i][j] == marker;
                } 
                if (2 - i) == j {
                    winur = winur && self.array[i][j] == marker;
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

    fn get_state(&self) -> State
    {
        if self.check_win(Marker::O) 
        {
            return State::WinO;
        }
        else if self.check_win(Marker::X)
        {
            return State::WinX;
        }

        if self.get_blanks().len() == 0
        {
            return State::Draw;
        }
        else
        {
            return State::Incomplete;
        }
    }

    fn get_blanks(&self) -> Vec<(usize, usize)>
    {
        let mut empties = Vec::new();
        for i in 0..3
        {
            for j in 0..3
            {
                if self.array[i][j].is_empty()
                {
                    empties.push((i, j));
                }
            }
        }

        //println!("{:?}", empties);
        empties
    }

    fn calc_optimal(&self, marker : Marker) -> ((usize, usize), i32)
    {  
        let mut move_list = Vec::new();
        for i in 0..3
        {
            for j in 0..3
            {
                if self.array[i][j].is_empty()
                {
                    let mut mboard = Board{
                        array : self.array
                    };
                    mboard.array[i][j] = marker;
                    let state = mboard.get_state();

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
                        let score = -1 * mboard.calc_optimal(marker.opposite()).1; 
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

    fn comp_add(&mut self, marker : Marker)
    {
        println!("{}: Computing...", marker.to_color());
        let spot : (usize,  usize) = self.calc_optimal(marker).0;
        self.array[spot.0][spot.1] = marker; 
    }

    fn rand_add(&mut self, marker : Marker)
    {
        let empties = self.get_blanks();
        let pos = empties.choose(&mut rand::thread_rng()).unwrap();

        self.array[pos.0][pos.1] = marker;
    }

    fn turns(&mut self, 
        add_x : &dyn Fn(&mut Board, Marker),
        add_o : &dyn Fn(&mut Board, Marker)) -> State
    {
        let first = Marker::random();
        self.print();
        loop
        {
            if first == Marker::X
            {
                if self.get_state() == State::Incomplete { add_x(self, Marker::X); }
                self.print();
                if self.get_state() == State::Incomplete { add_o(self, Marker::O); }
                self.print();
            }
            if first == Marker::O
            {
                if self.get_state() == State::Incomplete { add_o(self, Marker::O); }
                self.print();
                if self.get_state() == State::Incomplete { add_x(self, Marker::X); }
                self.print();
            }
            let state = self.get_state();
            if state != State::Incomplete 
            { 

                self.clear();
                return state; 
            }     
        }
    }

    fn run_game(&mut self, 
                    add_x : &dyn Fn(&mut Board, Marker),
                    add_o : &dyn Fn(&mut Board, Marker))
    { //Complicated function reference moment
        let mut xwins = 0;
        let mut owins = 0;
        let mut draws = 0;
        'game: loop
        {
            match self.turns(add_x, add_o)
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

    fn clear(&mut self)
    {
        let mut enumerate = 0;
        for i in (0..3).rev()
        {
            for j in 0..3
            {
                enumerate += 1; 
                self.array[i][j] = Marker::Empty(enumerate);
            }
        }
    }

    fn new() -> Board
    {
        let mut newb = Board{
            array : [[Marker::Empty(0); 3]; 3]
        };
        newb.clear();
        return newb;
    }

    fn print(&self)
    {
        print!("\x1B[1J\x1B[1;1H");
        let mut enumerate = 0;
        for row in self.array { 
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

    fn choose_player(marker : Marker) -> &'static dyn Fn(&mut Board, Marker)
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
                "U" | "u" => return &Board::user_add,
                "C" | "c" => return &Board::comp_add,
                "R" | "r" => return &Board::rand_add,
                _ => {
                    println!("Input could not be read. Please try again.");
                    continue;
                }
            }
        }
    } 
}

fn main() {
    #[cfg(windows)]
    {
        control::set_virtual_terminal(true).unwrap();
    }

    let mut board = Board::new();

    board.run_game(Board::choose_player(Marker::X), Board::choose_player(Marker::O));
}
