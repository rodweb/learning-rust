use std::mem;

fn scope_and_shadowing() {
    println!("\nScope and shadowing:");
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outer, a = {}", a);
}

fn operators() {
    // arithmetic
    let mut a = 2+3*4;
    println!("{}", a);
    a += 2;
    println!("{}", a);

    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                   // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10; // 1    0   0   0   0  0  0  0 0 0
                             // 1024 512 256 128 64 32 16 8 4 2
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!("pi is less than 4? {}", pi_less_4);

    let x = 5;
    let is_x_5 = x == 5;
    println!("is x 5? {}", is_x_5);
}

fn declarations() {
    // immutable unsigned 0..255
    let a:u8 = 123; //8bits
    println!("a = {}", a);

    // mutable
    let mut b:u8 = 0;
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; // 32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification", c);

    // possible integers
    // i8 u8 i16 u16 i32 u32 i64 u64

    // size of system address
    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double-precision, 8 bytes or 64 bits, f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // true false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}


fn main() {
    declarations();
    operators();
    scope_and_shadowing();
}
