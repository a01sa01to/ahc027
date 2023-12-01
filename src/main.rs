use proconio::input;

enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

fn can_move(
    (i, j): (usize, usize),
    direction: Direction,
    wall_horizontal: &Vec<Vec<char>>,
    wall_vertical: &Vec<Vec<char>>,
) -> bool {
    match direction {
        Direction::Up => {
            if i == 0 {
                false
            } else {
                wall_horizontal[i - 1][j] == '.'
            }
        }
        Direction::Right => {
            if j == wall_vertical[i].len() {
                false
            } else {
                wall_vertical[i][j] == '.'
            }
        }
        Direction::Down => {
            if i == wall_horizontal.len() {
                false
            } else {
                wall_horizontal[i][j] == '.'
            }
        }
        Direction::Left => {
            if j == 0 {
                false
            } else {
                wall_vertical[i][j - 1] == '.'
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        wall_horizontal: [[char; n]; n - 1],
        wall_vertical: [[char; n - 1]; n],
        dirtiness: [[u32; n]; n]
    };
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
