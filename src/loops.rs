pub fn loop_fn(skip_val: usize) {
    // .skip will skip the index value in the vector (not the index when enumerating)
    // skipping 0 means, nothing will be skipped
    // skipping 1 means, 'a' will be skipped and so on....
    for (i, v) in ['a', 'b', 'c', 'd'].iter().enumerate().skip(skip_val) {
        println!("[{}] {}", i, v);
    }
}
