use crate::board_states::gen_new;

pub mod board_states;


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
    fn clone(&self) -> Path{
        Path {
            score: self.score,
            directions: self.directions.clone(),
            board: self.board.clone(),
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
    fn curr_path(&self) -> Vec<u8>{
        self.directions.clone()
    }

}


fn main(){
    let mut board = board_states::create_board(4_usize);
    let mut k = 0;
    while k < 1{
        k+=1;
        println!("before update {:?}",board);
        let mut optimal_path: Path = Path::new(vec![],board.clone());
        let mut optimal_score = optimal_path.curr_score();
        while optimal_path.curr_path().len() < 10000{
            let mut new_path = optimal_path.clone();
            let mut new_optimal = new_path.clone();
            let mut score = 0;
            for i in 0..4{
                new_path = new_path.clone().dir(i);
                if new_path.curr_score() > score{
                    new_optimal = new_path.clone();
                }
            }
            optimal_path = new_optimal.clone();

        }

//        for i in 0..4{
//
//            let i_path:Path = Path::new(vec![i],board.clone());
//            if i_path.curr_score() > optimal_score{
//                optimal_score = i_path.curr_score();
//                optimal_path = i_path.clone();
//            }
//
//            for j in 0..4{
//                let j_path= i_path.clone().dir(j);
//                if j_path.curr_score() > optimal_score{
//                    optimal_score = j_path.curr_score();
//                    optimal_path = j_path.clone();
//                }
//
//                for k in 0..4{
//
//                    let k_path= j_path.clone().dir(k);
//                    if k_path.curr_score() > optimal_score{
//                        optimal_score = k_path.curr_score();
//                        optimal_path = k_path.clone();
//                    }
//
//                    for l in 0..4{
//
//                        let l_path= k_path.clone().dir(l);
//                        if l_path.curr_score() > optimal_score{
//                            optimal_score = l_path.curr_score();
//                            optimal_path = l_path.clone();
//                        }
//
//
//                    }
//                }
//            }
//        }
        let path = optimal_path.curr_path();
        println!("{:?}",path);
        let mut new_board = board.clone();
        for k in 0..path.len(){
            new_board = board_states::gen_new(new_board.clone(),path[k]);
        }
        println!("{:?}",new_board);

    }

}


fn count_zeros(board: Vec<Vec<i32>>) -> i32 {
    let mut score = 0;
    for i in 0..board.len(){
        for k in 0..board[i].len(){
            if board[i][k] == 0{
                score +=0;
            }
            else{
                score+=board[i][k];
            }

        }
    }
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



