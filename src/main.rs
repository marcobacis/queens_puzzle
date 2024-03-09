use queens::Board;
use std::env;

fn main() {
    let mut size = 8;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        size = args[1].parse().unwrap();
    }

    let solutions = solve(size);
    println!("Found {} solutions", solutions);
}

fn solve(size: usize) -> usize {
    solve_helper(size, vec![])
}

// the "current" vector represents the current valid solution,
// expressed only with x coordinates
// Example:
// [1,3,0,2] with size = 5 (so we miss the final position)
// is . Q . . .
//    . . . Q .
//    Q . . . .
//    . . Q . .

fn solve_helper(size: usize, current: Vec<usize>) -> usize {
    let height = current.len();
    let queens = (0..height)
        .zip(current.iter())
        .map(|(y, x)| (y, *x))
        .collect();

    let board = Board::rectangle(size, current.len(), queens);
    if !board.is_valid() {
        return 0;
    }
    if current.len() == size {
        // Solution found! Print it or return it or something else
        return 1;
    }

    // Could reduce the number of branches by not considering queens on the same
    // diagonals as the already positioned ones
    (0..size)
        .filter(|p| !current.contains(p))
        .map(|opt| solve_helper(size, current.iter().chain([&opt]).map(|p| *p).collect()))
        .sum()
}
