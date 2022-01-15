use std::vec::Vec;
use rand::seq::SliceRandom;
use std::io;
use colored::Colorize;
//use std::ascii::AsciiExt;

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
        println!("Which number to place at? ");
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

        let x = ((num-1) - (num-1) % 3) / 3;
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

    let top_list : Vec<((usize,usize), i32)> = move_list[bottom_index..].to_vec();
    
    *top_list.choose(&mut rand::thread_rng()).unwrap() //Return the one with the top score
}

fn comp_add<'a>(board : &mut[[&'a str; 3]; 3], marker : &'a str)
{
    let spot : (usize,  usize) = calc_optimal(board, marker).0;
    board[spot.0][spot.1] = marker; 
}

#[allow(dead_code)]
fn rand_add<'a>(board : &mut[[&'a str; 3]; 3], marker : &'a str)
{
    let empties = get_blanks(board);
    let pos = empties.choose(&mut rand::thread_rng()).unwrap();
    
    board[pos.0][pos.1] = marker;
}

fn turns(board : &mut[[& str; 3]; 3]) -> State
{
    let markers = ["O","X"];
    let first = markers.choose(&mut rand::thread_rng()).unwrap(); //Choose randomly between markers
    print_board(board);
    loop
    {
        if *first == "X"
        {
            if get_state(board) == State::Incomplete { user_add(board, "X"); }
            print_board(board);
            if get_state(board) == State::Incomplete { comp_add(board, "O"); }
            print_board(board);
        }
        if *first == "O"
        {
            if get_state(board) == State::Incomplete { comp_add(board, "O"); }
            print_board(board);
            if get_state(board) == State::Incomplete { user_add(board, "X"); }
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

fn run_game(board : &mut[[& str; 3]; 3])
{
    let mut xwins = 0;
    let mut owins = 0;
    let mut draws = 0;
    'game: loop
    {
        match turns(board)
        {
            State::WinO => owins += 1,
            State::WinX => xwins += 1,
            State::Draw => draws += 1,
            _ => println!("Bruh")
        }
        println!("X: {}, O: {}, Draw: {}", xwins, owins, draws);
        
        loop
        {
            let mut response = String::new();
            println!("Play again? [Y/n]");
            io::stdin()
                .read_line(&mut response)
                .expect("Failed to read line");
        
            let response = response.trim();

            if response.eq_ignore_ascii_case("y") || response == ""
            {
                break;
            }
            else
            {
                break 'game;
            }
        }
    }


}

fn clear_board(board : &mut[[& str; 3]; 3])
{
    //let mut enumerate = 0;
    //for i in 0..3
    //{
    //    for j in 0..3
    //    {
    //        enumerate += 1;
    //        let tmp = format!("{}", enumerate);
    //        board[i][j] = &tmp;//.as_str();
    //    }
    //}

    *board = [["1", "2", "3"], 
             ["4", "5", "6"], 
             ["7", "8", "9"]];
}

fn new_board<'a>() -> [[&'a str; 3]; 3]
{
    let mut board = [[" "; 3]; 3];
    clear_board(&mut board);
    return board;
}

fn print_board(board : &[[&str; 3]; 3])
{
    print!("\x1B[1J\x1B[1;1H");
    let mut enumerate = 0;
    for row in board { 
        for spot in row
        {
            enumerate += 1;
            if *spot == "X"
            {
                print!(" {} ", spot.bold().red());
            }
            else if *spot == "O"
            {
                print!(" {} ", spot.bold().blue());
            }
            else
            {
                print!(" {} ", spot.bold());
            }
            if enumerate % 3 != 0
            {
                print!("{}", "|".bold());
            }
        }

        println!("");
        if enumerate != 9 {println!("{}", format!("---+---+---").bold());}
    }
}

fn main() {
    let mut board = new_board();

    run_game(&mut board);
}
