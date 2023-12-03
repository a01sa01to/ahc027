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
        dirtiness: [[usize; n]; n]
    };

    // 何回訪れたいか決める
    let mut num_visit = dirtiness.clone();
    let max_len = 100_000;
    // 周りを見て不可能なものは減らす
    {
        for i in 0..n {
            for j in 0..n {
                let mut dig = 0;
                if can_move((i, j), Direction::Up, &wall_horizontal, &wall_vertical) {
                    dig += num_visit[i - 1][j];
                }
                if can_move((i, j), Direction::Right, &wall_horizontal, &wall_vertical) {
                    dig += num_visit[i][j + 1];
                }
                if can_move((i, j), Direction::Down, &wall_horizontal, &wall_vertical) {
                    dig += num_visit[i + 1][j];
                }
                if can_move((i, j), Direction::Left, &wall_horizontal, &wall_vertical) {
                    dig += num_visit[i][j - 1];
                }
                num_visit[i][j] = num_visit[i][j].min(dig / 2);
            }
        }

        let mut len = 0;
        for i in 0..n {
            for j in 0..n {
                len += num_visit[i][j];
            }
        }

        if len > max_len {
            for i in 0..n {
                for j in 0..n {
                    num_visit[i][j] = num_visit[i][j] * max_len / len;
                }
            }
        }
    }

    // 行動の長さを決める
    let num_visit = num_visit;
    let mut len = 0;
    for i in 0..n {
        for j in 0..n {
            len += num_visit[i][j];
        }
    }
    assert!(
        len <= max_len,
        "len is {}, which is larger than max_len {}",
        len,
        max_len
    );

    // いつ行きたいか求める
    let mut wants_turn = vec![vec![Vec::<usize>::new(); n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..num_visit[i][j] {
                wants_turn[i][j].push(k * len / num_visit[i][j]);
            }
        }
    }

    // いい感じに求める
    let mut ans = vec![(!0, !0); len];

    // output
    for i in 0..len {
        let (nowi, nowj) = ans[i];
        let (nxti, nxtj) = ans[(i + 1) % len];
        assert!(
            (nowi != nxti && nowj == nxtj && (nowi + 1 == nxti || nowi == nxti + 1))
                || (nowi == nxti && nowj != nxtj && (nowj + 1 == nxtj || nowj == nxtj + 1)),
            "[turn {}] not continuous: ({}, {}) -> ({}, {})",
            i,
            nowi,
            nowj,
            nxti,
            nxtj
        );
        let dir = {
            if nowi + 1 == nxti {
                Direction::Down
            } else if nowi == nxti + 1 {
                Direction::Up
            } else if nowj + 1 == nxtj {
                Direction::Right
            } else if nowj == nxtj + 1 {
                Direction::Left
            } else {
                unreachable!()
            }
        };
        match dir {
            Direction::Up => print!("U"),
            Direction::Right => print!("R"),
            Direction::Down => print!("D"),
            Direction::Left => print!("L"),
        }
    }
    println!();
}
