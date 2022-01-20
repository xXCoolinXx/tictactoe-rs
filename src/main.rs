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

mod game;

use game::{Board, Marker};

fn main() {
    #[cfg(windows)]
    {
        control::set_virtual_terminal(true).unwrap();
    }

    let mut board = Board::new();

    board.run_game(Board::choose_player(Marker::X), Board::choose_player(Marker::O));
}
