fn get_sum(a: i64, b: i64) -> i64 {
    if a == b {
        return a;
    }
    println!("{} {}", a, b);

    let fast_a = a / 2 * a.abs() + if a & 1 == 1 { a } else { a / 2 };
    let fast_b = b / 2 * b.abs() + if b & 1 == 1 { b } else { b / 2 };

    if a.signum() == b.signum() {
        if (a.signum() > 0 && a.abs() > b.abs()) || (a.signum() < 0 && a.abs() > b.abs()) {
            return fast_a - fast_b + b;
        } else {
            return fast_b - fast_a + a;
        }
    }

    fast_a + fast_b
}
