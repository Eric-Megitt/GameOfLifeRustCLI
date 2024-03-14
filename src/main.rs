use std::{thread::sleep, time::Duration};

const SIZE: usize = 10; 

fn main() {
    let mut board: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    //init
    board[5][5] = 1;
    board[5][6] = 1;
    board[5][7] = 1;

    //simulation
    loop {
        print_board(board);
        tick(&mut board);
        sleep(Duration::from_secs(1));
        clearscreen::clear();
    }
}

fn tick(board: &mut[[i32; SIZE]; SIZE]) {
    let mut adjecent_live_neighbours: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        for j in 0..SIZE {
            adjecent_live_neighbours[i][j] = count_live_neighbours(i as i32, j as i32, &board) as i32;
        }
    }

    for i in 0..SIZE {
        for j in 0..SIZE {
            match adjecent_live_neighbours[i][j] {
                2 => {},
                3 => board[i][j] = 1,
                _ => board[i][j] = 0,
            }
        }
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

fn count_live_neighbours(position_x: i32, position_y: i32, board: &[[i32; SIZE]; SIZE]) -> i32 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0_i32 && j == 0_i32 {
                continue;
            } 
                
            if position_x + i >= 0 && position_x + i < (SIZE as i32) - 1 && position_y + j >= 0 && position_y + j < (SIZE as i32) - 1 {
                if board[(position_x + i) as usize][(position_y + j) as usize] == 1 {
                    count += 1;
                }
            }
        }
    }
    
    count
}
