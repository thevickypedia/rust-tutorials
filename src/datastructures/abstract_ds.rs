use std::collections::VecDeque;

pub fn stack() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.pop();
}

pub fn queue() {
    let mut queue = VecDeque::new();
    queue.push_back(1);
    queue.pop_front();
}
