fn main() {
    proconio::input! {
        n: i64,
        a: i64,
        b: i64,
        p: i64,
        q: i64,
        r: i64,
        s: i64,
    };

    let (max1, min1) = (i64::max(1 - a, 1 - b), i64::min(n - a, n - b));
    let (max2, min2) = (i64::max(1 - a, b - n), i64::min(n - a, b - 1));

    let k1 = (max1..=min1).into_iter()
                          .map(|k| (a + k, b + k))
                          .collect::<Vec<(i64, i64)>>();

    let k2 = (max2..=min2).into_iter()
                          .map(|k| (a + k, b - k))
                          .collect::<Vec<(i64, i64)>>();

    for column in p..=q {
        let contains = |point| k1.contains(&point) || k2.contains(&point);

        let result = (r..=s).into_iter()
                            .map(|row| contains((column, row)))
                            .map(|ans| if ans {'#'} else {'.'})
                            .collect::<String>();

        println!("{}", result);
    }
}
