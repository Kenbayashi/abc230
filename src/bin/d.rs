fn main() {
    proconio::input! {
        n: i64,
        d: i64,
        mut walls: [(i64, i64); n],
    };

    walls.sort_by_key(|&(_, r)| r);

    let mut count = 0;

    while 0 != walls.len() {
        let &(_, right_edge) = walls.get(0).unwrap();
        let punch = right_edge + d - 1;

        walls = walls.into_iter()
                     .filter(|&(left_edge, _)| punch < left_edge)
                     .collect::<Vec<(i64, i64)>>();

        count += 1;
    }

    println!("{}", count);
}
