use csv::{Reader};
use std::str::Chars;

pub fn get_data(path: &str) -> Vec<([[u8;9];9], [[u8;9];9])> {
    match Reader::from_path(path){
        Ok(mut rdr) => {
            let mut return_value: Vec<([[u8;9];9], [[u8;9];9])> = vec![];

            for result in rdr.records(){
                match result {
                    Ok(line) => {
                        return_value.push((generate_board(line.get(0).unwrap().chars() as Chars), generate_board(line.get(1).unwrap().chars() as Chars)));
                    },
                    Err(e) => println!("{:?}", e),
                }
            }
            return return_value;
        },
        Err(e) => println!("Error {:?}", e)
    }

    return vec![];
}

fn generate_board(mut line: Chars) -> [[u8;9];9] {
    let mut chars: [u8;81] = [0;81];
    let mut board: [[u8;9];9] = [[0;9];9];

    for i in 0..81 {
        match line.next() {
            Some(value) => chars[i] = value.to_digit(10).unwrap() as u8,
            None => println!("There are not enough values inside this board")
        }
    }

    let mut count: usize = 0;
    for i in 0..9 {
        for j in 0..9 {
            if chars[count] != 0 {
                board[i][j] = chars[count];
            }
            count += 1;
        }
    }
    board
}
