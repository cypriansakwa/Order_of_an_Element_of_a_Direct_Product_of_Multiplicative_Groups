use num::integer::lcm;

// Function to compute the multiplicative order of a modulo n
fn multiplicative_order(a: u64, n: u64) -> u64 {
    let mut order = 1;
    let mut power = a % n;
    while power != 1 {
        power = (power * a) % n;
        order += 1;
    }
    order
}

// Function to compute the order of (a, b) in Z_n^* x Z_m^*
fn order(a: u64, n: u64, b: u64, m: u64) -> u64 {
    let order_a = multiplicative_order(a, n);
    let order_b = multiplicative_order(b, m);
    lcm(order_a, order_b)
}

fn main() {
    let a = 16;
    let n = 23;
    let b = 43;
    let m = 52;

    let result = order(a, n, b, m);
    println!("The order of ({}, {}) in Z_{}^* x Z_{}^* is {}", a, b, n, m, result);
}


