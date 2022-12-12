fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(numbers: Vec<u64>) -> u64 {
    let mut lcm = numbers[0];
    for i in 1..numbers.len() {
        lcm = lcm * numbers[i] / gcd(lcm, numbers[i]);
    }
    lcm
}
