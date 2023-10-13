fn is_prime(n:u32)->bool{
    if n<1{
        return false;
    }
    for i in 2..n/2{
        if n%i==0{
            return false;
        }
    }
    true
}

fn next_prime(n: u32) -> u32 {
    let mut i = n;
    while !is_prime(i) {
        i += 1;
    }
    i
}

fn main() {
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(10), false);
    assert_eq!(next_prime(8), 11);
    assert_eq!(next_prime(24), 29);
    println!("next_prime for 12: {}",next_prime(12));
    println!("next_prime for 24: {}",next_prime(24));
    println!("next_prime for 11: {}",next_prime(11));

}
