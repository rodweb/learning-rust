#![allow(unused_variables)]
#![allow(dead_code)]

use std::mem;

struct Point {
    x:f64,
    y:f64,
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p = Point{ x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point{ x: 5.0, y: 10.0 };

    let line = Line{ start: p, end: p2 };
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), // tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black: u8}, // struct
}

fn enums() {
    //let c:Color = Color::Red;
    //let c:Color = Color::RgbColor(10,0,0);
    let c:Color = Color::CmykColor{cyan: 0, magenta: 0, yellow: 0, black: 255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0)
        | Color::CmykColor{cyan:_,magenta:_,yellow:_,black:255} => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r,g,b),
        _ => ()
    }
}

fn option() {
    let x = 3.0;
    let y = 2.0;
    //let y = 0.0;

    let result:Option<f64> =
        if y != 0.0 { Some(x/y) } else { None };

    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x,y,z),
        None => println!("cannot divide {} by {}", x,y),
    }

    if let Some(z) = result { println!("z = {}", z); }
}

fn array() {
  let mut a:[i32;5] = [1,2,3,4,5];

  println!("a has {} elements, first is {}", a.len(), a[0]);

  a[0] = 321;
  //a[0] = 1;

  println!("{:?}", a); // print whole array

  if a != [1,2,3,4,5] {
    println!("does not match");
  }

  let b = [1; 10]; // b.len() == 10

  for i in 0..b.len() {
    println!("{}", b[i]);
  }

  println!("b took up {} bytes", mem::size_of_val(&b));

  let mtx:[[f32;3]; 2] = [
    [1.0, 0.0, 0.0],
    [0.0, 2.0, 0.0],
  ];

  println!("{:?}", mtx);

  for i in 0..mtx.len() {
    for j in 0..mtx[i].len() {
      if i == j {
        println!("[{}][{}] = {}", i, j, mtx[i][j]);
      }
    }
  }
}

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
    println!("a[0] = {}", a[0]);

    // must be usize if used as an index
    let idx:usize = 0;
    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    match a.get(3) { // option
        Some(x) => println!("a[3] = {}", x),
        None => println!("no such element")
    }

    for x in &a { println!("{}", x); }

    a.push(44);
    println!("{:?}", a);

    let last_elem = a.pop(); // option

    println!("last elem is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn use_slice(slice: &mut[i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 1234;
}

fn slices() {
    println!("\nSlices:");
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]);
    println!("{:?}", data);
}

fn strings() {
    println!("\nStrings:");

    // utf-8
    let s:&'static str = "hello there!"; // &str = string slice

    for c in s.chars() { // s.chars().rev()
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first char is {}", first_char);
    }

    // heap String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    let u:&str = &letters;

    // let z = letters + &letters;
    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");

    println!("{}", abc.replace("ello", "goodbye"));
}

fn main() {
    structures();
    enums();
    option();
    array();
    vectors();
    slices();
    strings();
}
