mod board;
mod solver;

fn main() {
    let mut board = board::Board::parse(
        "8.1,1.1,1.1,1,1,1.1,1.1,1.1,3,1.1,5.1,2.8,1",
        "10.1,1.1,1.1,1,2,1.1,2,1.1,1,2,1.1,1,1.10.1.1",
    )
    .unwrap();
    println!("Start: \n{}", board);
    board.low_hanging();
    println!("2: \n{}", board);
    board.solve_step();
    println!("3: \n{}", board);
    board.solve_step();
    println!("End: \n{}", board);
    println!("Ok")
}
