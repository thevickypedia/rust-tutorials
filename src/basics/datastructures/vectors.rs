pub fn fixed() {
    let vector = vec!["one", "two", "three", "four", "five", "six"];
    let first = vector[0];
    println!("First element: {}", first);
    let sliced = &vector[2..5]; // & is required to have a size known at compile-time
    println!("Sliced elements [2-5]: {:?}", sliced);
}

pub fn mutable() {
    let mut vector = vec!["one", "two", "three"];
    vector.pop(); // pop the last element
    vector.remove(1);  // remove 1st index (second)
    vector.push("four");  // insert element to the last index
    vector.insert(0, "zero");  // insert element 0th index (zero)
    println!("Restructured mutable vector: {:?}", vector);
}
