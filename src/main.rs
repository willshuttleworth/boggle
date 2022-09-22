//replace all type casts with ru or cu
//make board array instead of vector?
use array2d::Array2D;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();  
    let fname = match args.get(1) {
        Some(name) => name,
        None => panic!("give filename as command line arg pls"),
    };
    let mut board = load_board(fname);
    let len = board.num_rows() as i8;
    //write function to do dfs for every cell in board
    //nested for loops to do this function on every cell
    //function will take board as param, return nothing but board passed in will be mutable?
    for i in 0..len {
        for j in 0..len {
            //call dfs for every cell here 
            dfs(i, j, String::new(), &mut board, len);
        }
    }
}

fn dfs(r: i8, c: i8, mut word: String, board: &mut Array2D<char>, len: i8) -> () {
    let ru = r as usize;
    let cu = c as usize;
    word.push(board[(ru, cu)]);
    println!("{}", word);
    let curr = board[(ru, cu)];
    board[(ru, cu)] = '-';

    //north, r-1, c
    if (r - 1 >= 0) && (board[(ru - 1, cu)] != '-'){
        dfs(r - 1, c, String::from(&word[..]), board, len);
    }
        
    //northeast, r-1, c+1
    if (r - 1 >= 0) && (c + 1 < len) && (board[(ru - 1, cu + 1)] != '-'){
        dfs(r - 1, c + 1, String::from(&word[..]), board, len);
    }
    
    //east, r, c+1
    if (c + 1 < len) && (board[(ru, cu + 1)] != '-'){
        dfs(r, c + 1, String::from(&word[..]), board, len);
    }
    
    //southeast, r+1, c+1
    if (r + 1 < len) && (c + 1 < len) && (board[(ru + 1, cu + 1)] != '-'){
        dfs(r + 1, c + 1, String::from(&word[..]), board, len);
    }
    
    //south, r+1, c
    if (r + 1 < len) && (board[(ru + 1, cu)] != '-'){
        dfs(r + 1, c, String::from(&word[..]), board, len);
    }
    
    //southwest, r+1, c-1
    if (r + 1 < len) && (c - 1 >= 0) && (board[(ru + 1, cu - 1)] != '-'){
        dfs(r + 1, c - 1, String::from(&word[..]), board, len);
    }
    
    //west, r, c-1
    if (c - 1 >= 0) && (board[(ru, cu - 1)] != '-'){
        dfs(r, c - 1, String::from(&word[..]), board, len);
    }
    
    //northwest, r-1, c-1
    if (r - 1 >= 0) && (c - 1 >= 0) && (board[(ru - 1, cu - 1)] != '-'){
        dfs(r - 1, c - 1, String::from(&word[..]), board, len);
    }

    board[(ru, cu)] = curr;
}

fn load_board(fname: &str) -> Array2D<char> {
    let mut board: Vec<Vec<char>> = Vec::new(); 

    let file = File::open(fname);
    let mut file = match file {
        Ok(file) => file,
        Err(error) => panic!("could not open specified file: {:?}", error),
    };
    
    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not convert input file to string");
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        board.push(row); 
    }
    let len = board.len();
    let mut the_board = Array2D::filled_with('-', len, len);
    for i in 0..len {
        for j in 0..len {
            the_board[(i, j)] = board[i][j];
        }
    }
    the_board
}

#[test]
fn tester() {
    assert_eq!("ur mom", "ur mom");
}
