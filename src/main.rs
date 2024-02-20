

pub mod board_states;

#[derive(Clone)]
struct Path {
    score: i32,
    directions: Vec<u8>,
    board: Vec<Vec<i32>>,
}

impl Path{
    fn new(directions: Vec<u8>,board: Vec<Vec<i32>>) -> Path{
        Path {
            score: 0,
            directions,
            board,
        }


    }
    fn dir(&self, dir: u8) -> Path{
        let new_board = board_states::gen_new(self.board.clone(),dir);
        let mut new_directions = self.directions.clone();

        new_directions.push(dir);
        Path {
            score: self.score + count_zeros(new_board.clone()),
            directions: new_directions,
            board: new_board,

        }
    }
    fn curr_board(&self) -> Vec<Vec<i32>> {
        self.board.clone()
    }
    fn curr_score(&self) -> i32{
        self.score.clone()
    }

}


fn main(){
    let mut max_score = 0;
    for k in 0..100{
        let score = agent_four_step_max();
        if score >= max_score{
            max_score = score;
            println!("score: {}, runs: {}",max_score, k+1);
        }
    }
}


fn agent_four_step_max() -> i32{
    let mut board = board_states::create_board(4_usize);
    let mut k = 0;
    let mut old_board:Vec<Vec<i32>> = Vec::new();
    while k < 10000 && old_board != board{
        old_board = board.clone();
        k+=1;
        let mut optimal_path: Path = Path::new(vec![],board.clone());
        let mut optimal_score = optimal_path.curr_score();
//        while optimal_path.curr_path().len() < 4{
//            let mut new_optimal = optimal_path.clone();
//            for i in 0..4{
//                let temp_path = optimal_path.clone().dir(i);
//
//                if temp_path.curr_score() > optimal_score{
//                    new_optimal = temp_path.clone();
//                    optimal_score = new_optimal.curr_score();
//                }
//            }
//            optimal_path = new_optimal.clone();
//
//        }
        for i in 0..4{

            let i_path:Path = Path::new(vec![],board.clone());
            let i_path = i_path.dir(i);
            let i_score = i_path.curr_score();
            if i_path.curr_score() > optimal_score{
                optimal_score = i_score;
                optimal_path = i_path.clone();
            }

            for j in 0..4{

                let j_path = i_path.clone().dir(j);
                let j_score = j_path.curr_score()+i_score;
                if j_score> optimal_score{
                    optimal_score = j_score;
                    optimal_path = j_path.clone();
                }

                for k in 0..4{

                    let k_path = j_path.clone().dir(k);
                    let k_score = j_score + k_path.curr_score();
                    if k_score > optimal_score{
                        optimal_score = k_score;
                        optimal_path = k_path.clone();
                    }

                    for l in 0..4{

                        let l_path = k_path.clone().dir(l);
                        let l_score = k_score + l_path.curr_score();
                        if l_score >  optimal_score{
                            optimal_score = l_score;
                            optimal_path = l_path.clone();
                        }


                    }
                }
            }
        }
        board = optimal_path.curr_board();

    }
    let mut max_val = 0;
    for i in 0..board.len(){
        for j in 0..board[0].len(){
            if board[i][j] > max_val{
                max_val = board[i][j];
            }
        }
    }

    max_val
}


fn count_zeros(board: Vec<Vec<i32>>) -> i32 {
    let mut score = 0;
    let mut zeros = 0;
    for i in 0..board.len(){
        for k in 0..board[i].len(){
            if board[i][k] != 0{
                score +=board[i][k];
            }
            else if board[i][k] == 0{
                zeros+=1;
            }

        }

    }
    score = score + 8*zeros;
    score
}

//    let mut test_board: Vec<Vec<i32>> = vec![vec![2,2,2,0],vec![0,2,2,4],vec![4,2,4,2],vec![0,0,0,0]];
//    let mut new_board = board_states::rotate_and_update(test_board.clone(),0);
//    new_board = gen_new(new_board,0);
//    let mut random_generator = rand::thread_rng();
//    while new_board != test_board{
//        let dir = random_generator.gen_range(0..4);
//        println!("{}",dir);
//        test_board = new_board.clone();
//        new_board = gen_new(new_board,dir);
//        let print_clone = new_board.clone();
//        for row in print_clone{
//            println!("{:?}",row);
//        }
//    }



