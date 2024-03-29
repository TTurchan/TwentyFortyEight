use rand::Rng;

pub fn create_board(dim: usize) -> Vec<Vec<i32>> {
    let mut board: Vec<Vec<i32>> = Vec::new();
    for _i in 0..dim{
        let mut row:Vec<i32> = Vec::new();
        for _j in 0..dim{
            row.push(0);
        }
        board.push(row);
    }
    let mut rng = rand::thread_rng();
    let random_i:usize = rng.gen_range(0..dim);
    let random_j:usize = rng.gen_range(0..dim);
    let new_num = rng.gen_range(1..=2);
    board[random_i][random_j] = 2*new_num;
    board
}

pub fn check_valid_moves(board: Vec<Vec<i32>>) -> bool{
    for k in 0..4{
        let cloned_board = rotate_and_update(board.clone(), k);
        if board != cloned_board{
            return true
        }

    }
    false
}


pub fn gen_new(board: Vec<Vec<i32>>,dir: u8) -> Vec<Vec<i32>> {
    let mut zeros:Vec<Vec<usize>> = Vec::new();
    let mut updated_board:Vec<Vec<i32>> = board.clone();
    updated_board = rotate_and_update(updated_board.clone(),dir);
    for i in 0..board.len(){
        for j in 0..board[0].len(){
            if board[i][j] == 0{
                zeros.push(vec![i,j]);
            }
        }

    }
    if zeros.len() == 0{
        return updated_board
    }
    let mut rng = rand::thread_rng();

    let random_ind = rng.gen_range(0..zeros.len());
    let new_num = rng.gen_range(1..=2);
    let index = &zeros[random_ind];

    updated_board[index[0]][index[1]] = 2*new_num;
    updated_board
}
pub fn rotate_and_update(board: Vec<Vec<i32>>, dir: u8) -> Vec<Vec<i32>> {
    if dir == 0{
        return update_board(board)
    }
    if dir == 1{
        //rotate once then update then rotate backwards
        let rows = board.len();
        let cols = if rows > 0 { board[0].len() } else { 0 };
        let mut rotated_board = vec![vec![0; rows]; cols]; // Create a new matrix with swapped dimensions

        for (i, row) in board.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                rotated_board[j][rows - 1 - i] = val; // Place elements in their new positions
            }
        }
        let board = update_board(rotated_board.clone());
        for (i,row) in board.iter().enumerate() {
            for (j,&val) in row.iter().enumerate() {
                rotated_board[rows-j-1][i] = val;
            }
        }
        return rotated_board
    }
    if dir == 2{
        //rotate once then update then rotate backwards
        let rows = board.len();
        let cols = if rows > 0 { board[0].len() } else { 0 };
        let mut rotated_board = vec![vec![0; rows]; cols]; // Create a new matrix with swapped dimensions

        for (i, row) in board.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                rotated_board[rows-1-j][i] = val; // Place elements in their new positions
            }
        }
        let board = update_board(rotated_board.clone());
        for (i,row) in board.iter().enumerate() {
            for (j,&val) in row.iter().enumerate() {
                rotated_board[j][rows-i-1] = val;
            }
        }
        return rotated_board
    }
    if dir == 3{
        let mut rotated_board = board;
        rotated_board.reverse();
        for row in rotated_board.iter_mut() {
            row.reverse();
        }
        let board = update_board(rotated_board.clone());
        let mut rotated_board = board;
        rotated_board.reverse();
        for row in rotated_board.iter_mut() {
            row.reverse();
        }
        rotated_board
    } else{
        board
    }
}


pub fn update_board(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result_vector: Vec<Vec<i32>> = Vec::new();

    for row in board {
        let mut processed_row: Vec<i32> = Vec::new();
        let mut ind = 0;
        let mut k = 0;
        let mut merge_flag = false;
        while k < row.len(){
            if row[k] ==0 {
                k+=1;
            }else if merge_flag == false && ind > 0 && processed_row[ind-1] == row[k]{
                processed_row[ind-1] = 2*processed_row[ind-1];
                merge_flag = true;
                k+=1;
            } else if merge_flag == true{
                processed_row.push(row[k]);
                k+=1;
                merge_flag = false;
                ind+=1;
            } else if ind == 0{
                processed_row.push(row[k]);
                ind+=1;
                k+=1;
            } else {
                processed_row.push(row[k]);
                ind+=1;
                k+=1;
                merge_flag = false;
            }
        }
        while ind < row.len(){
            processed_row.push(0);
            ind+=1;
        }
        result_vector.push(processed_row);

    }
    result_vector

}
