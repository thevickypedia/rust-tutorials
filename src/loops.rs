pub fn for_loop(skip_val: usize) {
    // .skip will skip the index value in the vector (not the index when enumerating)
    // skipping 0 means, nothing will be skipped
    // skipping 1 means, 'a' will be skipped and so on....
    for (i, v) in ['a', 'b', 'c', 'd'].iter().enumerate().skip(skip_val) {
        println!("[{}] {}", i, v);
    }
    while_loop()
}

fn while_loop() {
    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html?highlight=mutable#mutable-references
    let mut i = 0;  // Make 'i' mutable
    while i < 5 {
        i = i + 1;
        println!("while this is a while loop, i is still {}", i)
    }
}
