use proconio::input;

enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

fn main() {
    input! {
        n: usize,
        wall_horizontal: [[char; n]; n - 1],
        wall_vertical: [[char; n - 1]; n],
        dirtiness: [[u32; n]; n]
    };
    // initialize
    let mut graph = vec![vec![vec![false; 4]; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i < n - 1 && wall_horizontal[i][j] == '1' {
                graph[i][j][Direction::Down as usize] = true;
                graph[i + 1][j][Direction::Up as usize] = true;
            }
            if j < n - 1 && wall_vertical[i][j] == '1' {
                graph[i][j][Direction::Right as usize] = true;
                graph[i][j + 1][Direction::Left as usize] = true;
            }
        }
    }
    let graph = graph;
    let mut ans = Vec::<Direction>::new();

    // output
    for direction in ans {
        match direction {
            Direction::Up => print!("U"),
            Direction::Right => print!("R"),
            Direction::Down => print!("D"),
            Direction::Left => print!("L"),
        }
    }
    println!();
}
