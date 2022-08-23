pub fn run(board_original: [[u8;9];9]) -> [[u8;9];9]{
    let mut board : [[u8;9];9] = board_original;
    solve(&mut board);
    return board;
}

fn solve(board: &mut [[u8;9];9]) -> bool{
    // println!("Iteration");
    let find = find_empty(&board);

    let row: usize = find.0;
    let col: usize = find.1;

    if find.0 == 9 {
        return true;
    }

    for i in 1..=9 {
        if check_valid(row, col, i, &board){
            board[row][col] = i;

            if solve(board) {
                return true;
            }

            board[row][col] = 0;
        }
    }


    return false;
}

fn find_empty(board: &[[u8;9];9]) -> (usize, usize) {
    for i in 0..board.len() {
        for j in 0..board.len() {
            if board[i][j] == 0 {
                return (i, j);
            }
        }
    }
    (9, 9)
}

fn check_valid(i: usize, j: usize, new_value: u8, board: &[[u8;9];9]) -> bool{
    /* HORIZONTAL LINES */
    for item in board[i] {
        if item == new_value {
            return false;
        }
    }

    /* VERTICAL LINES */
    for index in 0..9 {
        if board[index][j] == new_value {
            return false;
        }
    }

    /* IN SQUARE */
    let index_horizontal = j-j%3;
    let index_vertical = i-i%3;
    let square_slice: &[[u8;9]] = &board[index_vertical..index_vertical + 3];

    for row in square_slice.iter() {
        if row[index_horizontal..index_horizontal + 3].contains(&new_value) {
            return false;
        }

    }
    return true;
}

pub fn print_board(board: &[[u8;9];9]) {
    for i in 0..board.len() {
        if i % 3 == 0 && i != 0 {
            println!("- - - - - - - - - - - - - ");
        }

        for j in 0..board.len() {
            if j % 3 == 0 && j != 0 {
                print!(" | ");
            }
            if j == 8 {
                println!("{}", board[i][j]);
            } else {
                print!("{} ", board[i][j]);
            }
        }
    }
}