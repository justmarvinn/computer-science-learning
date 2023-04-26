pub fn karatsuba(x: u128, y: u128) -> u128 {
    let mut x_str = x.to_string();
    let mut y_str = y.to_string();
    if x_str.len() != y_str.len() { return x * y; }
    // while x_str.len() < y_str.len() { x_str = String::from("0") + &x_str; }
    // while y_str.len() < x_str.len() { y_str = String::from("0") + &y_str; }
    // assert_eq!(x_str.len(), y_str.len());

    let n = x_str.len();
    if n == 1 { return x * y; }
    let m = n / 2;
    let a = x_str[..m].parse::<u128>().unwrap();
    let b = x_str[m..].parse::<u128>().unwrap();
    let c = y_str[..m].parse::<u128>().unwrap();
    let d = y_str[m..].parse::<u128>().unwrap();

    let p = a + b;
    let q = c + d;
    let ac = karatsuba(a, c);
    let bd = karatsuba(b, d);
    let pq = karatsuba(p, q);
    // let pq  = p * q;
    let adbc = pq - ac - bd;
    10_u128.pow(n as u32) * ac + 10_u128.pow((n/2) as u32) * adbc + bd
}
