use std::thread;

use proconio::{input, marker::Chars};

type Direction = usize;
const UP: Direction = 0;
const RIGHT: Direction = 1;
const DOWN: Direction = 2;
const LEFT: Direction = 3;

const DI: [i64; 4] = [-1, 0, 1, 0];
const DJ: [i64; 4] = [0, 1, 0, -1];

const MAX_LEN: usize = 100_000;

fn can_move(
    (i, j): (usize, usize),
    direction: Direction,
    wall_horizontal: &Vec<Vec<char>>,
    wall_vertical: &Vec<Vec<char>>,
) -> bool {
    match direction {
        UP => {
            if i == 0 {
                false
            } else {
                wall_horizontal[i - 1][j] == '0'
            }
        }
        RIGHT => {
            if j + 1 == wall_horizontal[0].len() {
                false
            } else {
                wall_vertical[i][j] == '0'
            }
        }
        DOWN => {
            if i + 1 == wall_vertical.len() {
                false
            } else {
                wall_horizontal[i][j] == '0'
            }
        }
        LEFT => {
            if j == 0 {
                false
            } else {
                wall_vertical[i][j - 1] == '0'
            }
        }
        _ => unreachable!(),
    }
}

fn dfs(
    ans: &mut Vec<(usize, usize)>,
    visited: &mut Vec<Vec<u32>>,
    cand: &mut Vec<(u32, (usize, usize))>,
    now: &mut (usize, usize),
    wall_horizontal: &Vec<Vec<char>>,
    wall_vertical: &Vec<Vec<char>>,
    dirtiness: &Vec<Vec<u32>>,
    dep: &mut u32,
) {
    if *dep > MAX_LEN as u32 {
        return;
    }

    let (i, j) = *now;
    visited[i][j] = *dep;
    for direction in 0..4 {
        if can_move((i, j), direction, wall_horizontal, wall_vertical) {
            let (ni, nj) = (
                (i as i64 + DI[direction]) as usize,
                (j as i64 + DJ[direction]) as usize,
            );
            let di = if visited[ni][nj] == 0 && *now != (0, 0) {
                1_000_000
            } else {
                (*dep - visited[ni][nj]) * dirtiness[ni][nj]
            };
            cand.push((di, (ni, nj)));
        }
    }

    if cand.len() == 0 {
        return;
    }

    cand.sort();
    let (_, (ni, nj)) = cand.last().unwrap();
    ans.push((*ni, *nj));
    *now = (*ni, *nj);
    *dep += 1;
    cand.clear();

    dfs(
        ans,
        visited,
        cand,
        now,
        wall_horizontal,
        wall_vertical,
        dirtiness,
        dep,
    );
}

fn real_main() {
    input! {
        n: usize,
        wall_horizontal: [Chars; n - 1],
        wall_vertical: [Chars; n],
        dirtiness: [[u32; n]; n]
    };

    // solve
    let mut ans = Vec::<(usize, usize)>::new();
    ans.push((0, 0));
    let mut visited = vec![vec![0; n]; n];
    let mut now = (0, 0);
    let mut cand = Vec::<(u32, (usize, usize))>::new();
    let mut dep = 0;
    dfs(
        &mut ans,
        &mut visited,
        &mut cand,
        &mut now,
        &wall_horizontal,
        &wall_vertical,
        &dirtiness,
        &mut dep,
    );

    // つじつまあわせ
    while let Some(&(i, j)) = ans.last() {
        if i == 0 && j == 0 {
            break;
        }
        ans.pop();
    }

    assert!(
        ans.last().unwrap() == &(0, 0),
        "ans.last() = {:?}",
        ans.last()
    );
    assert!(ans.len() - 1 <= MAX_LEN, "ans.len() = {}", ans.len());

    // output
    for i in 0..ans.len() - 1 {
        let (nowi, nowj) = ans[i];
        let (nxti, nxtj) = ans[(i + 1) % ans.len()];
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
                DOWN
            } else if nowi == nxti + 1 {
                UP
            } else if nowj + 1 == nxtj {
                RIGHT
            } else if nowj == nxtj + 1 {
                LEFT
            } else {
                unreachable!()
            }
        };
        match dir {
            UP => print!("U"),
            RIGHT => print!("R"),
            DOWN => print!("D"),
            LEFT => print!("L"),
            _ => unreachable!(),
        }
    }
    println!();
}

fn main() {
    // ローカル Stack Overflow 対策
    let handler = thread::Builder::new()
        .stack_size(1024 * 1024 * 1024)
        .spawn(|| {
            real_main();
        })
        .expect("can't spawn thread");

    handler.join().expect("something's wrong with the thread");
}
