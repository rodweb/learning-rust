#![allow(unused_variables)]
#![allow(dead_code)]

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

fn main() {
    structures();
    enums();
    option();
}
