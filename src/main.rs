use std::{thread::sleep, time::Duration};

const SIZE: usize = 10; 

fn main() {
    let mut board: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    //init
    board[5][5] = 1;

    //simulation
    loop {
        //tick();
        clearscreen::clear();
        print_board(board);
        sleep(Duration::from_secs(1));
    }
}

fn print_board(board: [[i32; SIZE]; SIZE]) {
    for i in 0_usize..SIZE {
        for j in 0_usize..SIZE {
            print!("{}", board[i as usize][j as usize]);
        }
        println!();
    }
}

fn count_live_neighbours(position_x: i32, position_y: i32, board: [[i32; SIZE]; SIZE]) -> i32 {
    let mut count = 0;
    for i in -1..1 {
        for j in -1..1 {
            if i == 0_i32 && j == 0_i32 {
                continue;
            } 
                
            if position_x + i >= 0 && position_y + j >= 0 && position_x + i < (SIZE as i32) && position_y + j < (SIZE as i32) {
                if board[(position_x + i) as usize][(position_y + j) as usize] == 1 {
                    count += 1;
                }
            }
        }
    }
    
    count
}
