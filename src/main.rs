use std::vec::Vec;
use rand::seq::SliceRandom;
use std::io;
use std::ascii::AsciiExt;

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
    let mut x : usize = 0;
    let mut y : usize = 0;
    loop
    {
        let mut pos = String::new();
        println!("Where to do you want to place a marker? (row, col)");
        io::stdin()
            .read_line(&mut pos)
            .expect("Failed to read line");
    
        let mut pos = pos.trim().split_once(",").unwrap();

        x = match pos.0.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        y = match pos.1.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if board[x-1][y-1] == " " {
            board[x-1][y-1] = marker;
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
            if board[i][j] == " "
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
            if board[i][j] == " "
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
        
            let mut response = response.trim();

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
    *board = [[" "; 3]; 3]
}

fn print_board(board : &[[&str; 3]; 3])
{
    print!("\x1B[1J\x1B[1;1H");
    println!("   1   2   3  ");
    let mut enumerate = 0;
    for row in board { 
        enumerate += 1;
        let mut row_str = String::from(format!("{} ", enumerate));
        for spot in row
        {
            row_str.push_str(&format!(" {} |", spot)[..]);
        }
        row_str.pop();
        println!("{}", row_str);
        if enumerate != 3 {println!("{}", "  ---+---+---");}
    }
}

fn main() {
    let mut board:[[&str; 3]; 3] = [[" "; 3]; 3];

    //board = [["O", "X", "O"],
    //            ["X", "O", "X"],
    //            [" ", "X", "O"]];

    //rand_add(&mut board, "O");
    //comp_add(&mut board, "X");

    //let mut owins = 0; 
    //let mut xwins = 0;
    //let mut draws = 0;
//
    //while owins + xwins + draws < 2500
    //{
    //    match run_game(&mut board)
    //    {
    //        State::WinO => owins += 1,
    //        State::WinX => xwins += 1,
    //        State::Draw => draws += 1,
    //        _ => println!("Bruh")
    //    }
    //}

    run_game(&mut board);

    //println!("owins: {}", owins);
    //println!("xwins: {}", xwins);
    //println!("draws: {}", draws);

    //print_board(&board);
    //println!("{}", check_win(&board, "O") || check_win(&board, "X"));
    //if get_state(&board) == State::Draw
    //{
    //    println!("Bruh");
    //}
}
