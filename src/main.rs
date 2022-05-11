use picross_solver_lib::solve;

fn main() {
    let (board, steps) = match solve(
        "8.1,1.1,1.1,1,1,1.1,1.1,1.1,3,1.1,5.1,2.8,1",
        "10.1,1.1,1.1,1,2,1.1,2,1.1,1,2,1.1,1,1.10.1.1",
    ) {
        Some(v) => v,
        None => return,
    };
    println!("After {steps} steps: \n{}", board);
    println!("Ok")
}
