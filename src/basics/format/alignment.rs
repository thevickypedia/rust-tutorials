pub fn left_aligned() {
    let text = "LEFT ALIGNED TEXT";
    println!("PlaceHolder: {:<50}", text);
}

pub fn right_aligned() {
    let text = "RIGHT ALIGNED TEXT";
    println!("PlaceHolder: {:>50}", text);
}

pub fn center_aligned() {
    let text = "CENTER ALIGNED TEXT";
    println!("PlaceHolder: {:^50}", text);
}
