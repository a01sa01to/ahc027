use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use proconio::{input, marker::Chars};

type Direction = usize;
const UP: Direction = 0;
const RIGHT: Direction = 1;
const DOWN: Direction = 2;
const LEFT: Direction = 3;

const DI: [i64; 4] = [-1, 0, 1, 0];
const DJ: [i64; 4] = [0, 1, 0, -1];

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
    (i, j): (usize, usize),
    ans: &mut Vec<(usize, usize)>,
    wants_turn: &mut Vec<Vec<(VecDeque<i32>, i32)>>,
    wall_horizontal: &Vec<Vec<char>>,
    wall_vertical: &Vec<Vec<char>>,
    turn: usize,
) -> bool {
    // 最後のターンは確定
    if turn == ans.len() + 1 {
        return (i, j) == (0, 0);
    }

    // 次に行く場所を探す
    let mut nxt = BinaryHeap::<Reverse<(i32, (usize, usize, i32))>>::new();
    for dir in 0..4 {
        if can_move((i, j), dir, wall_horizontal, wall_vertical) {
            let (ni, nj) = ((i as i64 + DI[dir]) as usize, (j as i64 + DJ[dir]) as usize);
            let offset = wants_turn[ni][nj].1;
            let wants = &mut wants_turn[ni][nj].0;
            if wants.is_empty() {
                continue;
            }
            let want = wants.pop_front().unwrap();
            if offset == 0 && (ni, nj) != (0, 0) {
                nxt.push(Reverse((0, (ni, nj, offset))));
            } else {
                nxt.push(Reverse((want + offset - turn as i32, (ni, nj, offset))));
            }
        }
    }
    if nxt.is_empty() {
        return false;
    }

    while let Some(Reverse((want1, (ni, nj, offset)))) = nxt.pop() {
        if offset == 0 {
            wants_turn[ni][nj].1 = turn as i32;
        }
        if dfs(
            (ni, nj),
            ans,
            wants_turn,
            wall_horizontal,
            wall_vertical,
            turn + 1,
        ) {
            ans[turn] = (ni, nj);
            return true;
        }
        wants_turn[ni][nj]
            .0
            .push_front(want1 - offset + turn as i32);
        wants_turn[ni][nj].1 = offset;
    }

    false
}

fn main() {
    input! {
        n: usize,
        wall_horizontal: [Chars; n - 1],
        wall_vertical: [Chars; n],
        dirtiness: [[i32; n]; n]
    };

    // 何回訪れたいか決める
    let max_len = 100_000usize;
    let mut num_visit = dirtiness.clone();
    // 周りを見て不可能なものは減らす
    {
        for i in 0..n {
            for j in 0..n {
                let mut deg = 0;
                if can_move((i, j), UP, &wall_horizontal, &wall_vertical) {
                    deg += dirtiness[i - 1][j];
                }
                if can_move((i, j), RIGHT, &wall_horizontal, &wall_vertical) {
                    deg += dirtiness[i][j + 1];
                }
                if can_move((i, j), DOWN, &wall_horizontal, &wall_vertical) {
                    deg += dirtiness[i + 1][j];
                }
                if can_move((i, j), LEFT, &wall_horizontal, &wall_vertical) {
                    deg += dirtiness[i][j - 1];
                }
                num_visit[i][j] = dirtiness[i][j].min(deg / 2);
            }
        }

        let mut len = 0;
        for i in 0..n {
            for j in 0..n {
                len += num_visit[i][j] as usize;
            }
        }

        if len > max_len {
            for i in 0..n {
                for j in 0..n {
                    num_visit[i][j] = num_visit[i][j] * max_len as i32 / len as i32;
                }
            }
        }
    }

    // 行動の長さを決める
    let num_visit = num_visit;
    let mut len = 0;
    for i in 0..n {
        for j in 0..n {
            len += num_visit[i][j] as usize;
        }
    }
    assert!(
        len <= max_len,
        "len is {}, which is larger than max_len {}",
        len,
        max_len
    );

    // いつ行きたいか求める
    let mut wants_turn = vec![vec![(VecDeque::<i32>::new(), 0); n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..num_visit[i][j] {
                wants_turn[i][j]
                    .0
                    .push_back(k * len as i32 / num_visit[i][j]);
            }
        }
    }

    // いい感じに求める
    let mut ans = vec![(!0, !0); len];
    ans[0] = (0, 0);
    dfs(
        (0, 0),
        &mut ans,
        &mut wants_turn,
        &wall_horizontal,
        &wall_vertical,
        0,
    );

    // output
    for i in 0..len + 1 {
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
