use std::vec::Vec;
use rand::{Rng, seq::SliceRandom, prelude::ThreadRng};
//use rand::Rng;
use std::io::{self, Write};
use colored::*;
use ndarray::prelude::*;

#[derive(PartialEq)]
pub enum State
{
    WinO,
    WinX,
    Incomplete,
    Draw,
}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum Marker
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
            Marker::X => String::from("X"),
            Marker::O => String::from("O"),
            Marker::Empty(size) => size.to_string(),
        }
    }
    fn as_color(&self) -> ColoredString
    {
        match self
        {
            Marker::X => self.string().bold().red(),
            Marker::O => self.string().bold().blue(),
            Marker::Empty(_) => self.string().bold(),
        }
    }
    fn opposite(&self) -> Marker
    {
        match self
        {
            Marker::X => Marker::O,
            Marker::O => Marker::X,
            Marker::Empty(size) => Marker::Empty(*size)
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
    fn is_empty(&self) -> bool
    {
        match self
        {
            Marker::X | Marker::O => false,
            Marker::Empty(_) => true
        }
    }
}

#[derive(Clone)]
pub struct Board
{
    array : Array2<Marker>,
    rng : ThreadRng,
    width : usize,
}

impl Board {
    fn avg_add(&mut self, marker : Marker)
    {
        for pos in self.get_blanks()
        {
            let mut oppboard = self.clone();
            let mut dubboard = self.clone();
            
            oppboard.array[pos] = marker.opposite();
            dubboard.array[pos] = marker;

            if dubboard.check_win(marker) || oppboard.check_win(marker.opposite())
            {
                self.array[pos] = marker;
                return;
            } //If marker wins or opponent wins, place it there 
        }
        //If there is no win/lose, just put it in a random spot
        self.rand_add(marker);
    }

    fn calc_optimal(&mut self, marker : Marker, layer : u32) -> ([usize; 2], i32)
    {  
        let mut move_list = Vec::new();
        for (num, _) in self.array.iter().enumerate()
        {
            let index = self.enumerate_to_index(num);
                if self.array[index].is_empty()
                {
                    let mut mboard = self.clone();
                    mboard.array[index] = marker;
                    let state = mboard.get_state();

                    if state == marker.win_con()
                    {
                        return (index, 1);
                    }
                    else if state == State::Draw
                    {
                        return (index, 0);
                    }
                    else
                    {
                        let score;
                        if layer < 9 {
                            score = - mboard.calc_optimal(marker.opposite(), layer + 1).1; 
                        }
                        else
                        {
                            score = 0;
                        }
                        //Reverse scores for O because no want win
                        move_list.push((index, score));
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
        let top_list : Vec<([usize; 2], i32)> = move_list[bottom_index..].to_vec();
        
        *top_list.choose(&mut self.rng).unwrap() //Return one with the top score
    }

    fn check_win(&self, marker : Marker) -> bool 
    {
        let mut winul : bool = self.array[[0, 0]] == marker;
        let mut winur : bool = self.array[[self.nrows() - 1, 0]] == marker;
        for i in 0..self.nrows()
        {
            let mut winrow : bool = self.array[[i, 0]] == marker;
            let mut wincol : bool = self.array[[0, i]] == marker;

            for j in 0..self.ncols()
            {
                winrow = winrow && self.array[[i, j]] == marker;
                wincol = wincol && self.array[[j, i]] == marker;
                if i == j
                {
                    winul = winul && self.array[[i, j]] == marker;
                } 
                if (self.nrows() - 1 - i) == j {
                    winur = winur && self.array[[i, j]] == marker;
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

    pub fn choose_player(marker : Marker) -> &'static dyn Fn(&mut Board, Marker)
    {
        loop {
            let mut mstr = String::new();
            print!("Player {} [{}ser/{}PU]: ", marker.as_color(), "U".underline(), "C".underline());
            let _ = io::stdout().flush();
            io::stdin()
                    .read_line(&mut mstr)
                    .expect("Failed to read line");

            match mstr.as_str().trim().chars().next()
            {
                Some(input ) => match input.to_string().as_str()
                {
                    "U" | "u" => return &Board::user_add,
                    "C" | "c" => {
                        loop {
                            let mut level = String::new();//└►
                            print!("└►Level: [{}verage/{}andom/{}od]: ", "A".underline(), "R".underline(), "G".underline());
                            let _ = io::stdout().flush();
                            io::stdin()
                                .read_line(&mut level)
                                .expect("Failed to read line");

                            match level.as_str().trim().chars().next()
                            {
                                Some(input) => match input.to_string().as_str()
                                {
                                    "R" | "r" => return &Board::rand_add,
                                    "A" | "a" => return &Board::avg_add,
                                    "G" | "g" => return &Board::comp_add,
                                    _ => {
                                        println!("Input could not be read. Please try again.");
                                        continue;
                                    }
                                }
                                None =>
                                {
                                    println!("Input could not be read. Please try again.");
                                    continue;
                                }
                            }
                        }
                    }
                    _ => {
                        println!("Input could not be read. Please try again.");
                        continue;
                    }
                },
                None => {
                    println!("Input could not be read. Please try again.");
                    continue;
                },
            }
        }
    }

    fn clear(&mut self)
    {
        for num in 0..self.array.len()
        {
            //Puts the numbers in the arrangment of the numpad
            let index = self.input_to_index(num + 1);
            self.array[index] = Marker::Empty(num + 1);
        }

        self.width = self.array[[0, self.ncols() - 1]].string().len();
    }

    fn comp_add(&mut self, marker : Marker)
    {
        println!("{}: Computing...", marker.as_color());
        let spot = self.calc_optimal(marker, 0).0;
        self.array[spot] = marker;
    }

    fn size(&self) -> usize {
        self.ncols() * self.nrows()
    }

    fn nrows(&self) -> usize {
        self.array.nrows()
    }

    fn ncols(&self) -> usize {
        self.array.ncols()
    }

    fn get_blanks(&self) -> Vec<[usize; 2]>
    {
        let mut empties = Vec::new();

        for (num, tile) in self.array.iter().enumerate()
        {
            if tile.is_empty() {
                empties.push(self.enumerate_to_index(num));
            }
        }

        empties
    }

    fn enumerate_to_index(&self, num : usize) -> [usize; 2]
    {   
        let y = num % self.ncols(); 
        let x = (num - y) / self.ncols();
        [x, y]
    }

    fn input_to_index(&self, num : usize) -> [usize; 2]
    {
        let num = num - 1;
        let y = num % self.ncols(); 
        let x = self.ncols() - 1 - (num - y) / self.ncols();
        
        [x, y]
    }

    fn get_state(&self) -> State
    {
        if self.check_win(Marker::O) {
            State::WinO
        }
        else if self.check_win(Marker::X) {
            State::WinX
        } else if self.get_blanks().is_empty() {
            State::Draw
        } else {
            State::Incomplete
        }
    }

    pub fn new() -> Board
    {
        let empty_row = [Marker::Empty(0); 3];
        let mut newb = Board{
            array : array![empty_row, empty_row, empty_row],
            rng : rand::thread_rng(),
            width : 0,
        };

        newb.clear();
        newb
    }

    fn print(&self)
    {
        print!("\x1B[1J\x1B[1;1H");

        let mut dashes = String::new();
        for _ in 0..(self.width+2) {
            dashes.push_str("-");
        }
        for (num, tile) in self.array.iter().enumerate() {
            if num != 0 && num % self.ncols() == 0 {
                println!("");
                for i in 0..self.ncols() {
                    print!("{}", dashes.bold());
                    if i != self.ncols() - 1 {
                        print!("{}", "+".bold());
                    } 
                }
                println!("");
            }

            print!(" {:>width$} ", tile.as_color(), width=self.width);
            
            if (num + 1) % self.ncols() != 0 {
                print!("{}", "|".bold());
            }
        }
        println!("");
    }

    fn rand_add(&mut self, marker : Marker)
    {
        let empties = self.get_blanks();
        let pos = empties.choose(&mut self.rng).unwrap();

        self.array[*pos] = marker;
    }

    pub fn run_game(&mut self, 
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
                Marker::X.as_color(), 
                            xwins, 
                Marker::O.as_color(), 
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

    fn turns(&mut self, 
        add_x : &dyn Fn(&mut Board, Marker),
        add_o : &dyn Fn(&mut Board, Marker)) -> State
    {
        let func_list = [add_x, add_o];
        let marker_list = [Marker::X, Marker::O];

        let one = self.rng.gen_range(0..2);
        let two = 1 - one;

        self.print();
        loop
        {
            if self.get_state() == State::Incomplete { 
                func_list[one](self, marker_list[one]); 
            }
            self.print();
            if self.get_state() == State::Incomplete { 
                func_list[two](self, marker_list[two]); 
            }
            self.print();

            let state = self.get_state();
            if state != State::Incomplete 
            { 
                self.clear();
                return state; 
            }     
        }
    }

    fn user_add(&mut self, marker : Marker)
    {
        loop
        {
            let mut pos = String::new();
            print!("{}: ", marker.as_color());
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
            let size = self.size();
            if !((1..=size).contains(&num))
            {
                println!("That is not a choice! Please enter again.");
                continue
            }

            let index = self.input_to_index(num);

            if self.array[index].is_empty() {
                self.array[index] = marker;
                break
            } else {
                println!("That spot is already taken! Please enter again.");
                continue
            }
        }
    } 
}