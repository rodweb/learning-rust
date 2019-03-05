#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap() {
    println!("\nStack and heap:");

    let p1 = origin(); // stack
    let p2 = Box::new(origin()); // heap

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // store the address

    let p3 = *p2; // follow the address
    println!("p3.x = {}", p3.x);
}
