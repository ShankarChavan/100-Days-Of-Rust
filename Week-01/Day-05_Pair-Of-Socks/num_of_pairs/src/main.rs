fn sock_pairs(socks: &str) -> i32 {
    """
    The function sock_pairs takes a string socks as input and returns the number of sock pairs. 
    It uses a HashMap to keep track of the count of each type of sock. 
    For each sock in the input string, it increments the count in the HashMap. 
    If the count is even, it means that a pair has been formed, so it increments the pairs variable.
    """
    let mut pairs = 0;
    let mut sock_counts = std::collections::HashMap::new();
    for sock in socks.chars() {
        let count = sock_counts.entry(sock).or_insert(0);
        *count += 1;
        if *count % 2 == 0 {
            pairs += 1;
        }
    }
    pairs
}



fn main(){
    assert_eq!(sock_pairs("AA"), 1);
    assert_eq!(sock_pairs("ABABC"), 2);
    assert_eq!(sock_pairs("CABBACCC"), 4);
    println!("number of socks for AA: {}",sock_pairs("AA"));
    println!("number of socks for ABABC: {}",sock_pairs("ABABC"));
    println!("number of socks for CABBACCC: {}",sock_pairs("CABBACCC"));
}