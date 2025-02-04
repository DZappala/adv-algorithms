fn main() {
    println!("{:?}", solve(7));
    println!("{:?}", solve(100));
}

const fn solve(n: u32) -> u32 {
    match n {
        0 => unreachable!(),
        1 => 1,
        _ => solve(n - 1) + n.ilog10(),
    }
}
