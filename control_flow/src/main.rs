fn if_statement() {
    let temp = 35;

    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10 {
        println!("really cold");
    } else {
        println!("temperature is OK");
    }

    // if is an expression
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("day is {}", day);
}

fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 { continue; }

        println!("x = {}", x);
    }

    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);

        if y == 1<<10 { break; }
    }
}

fn for_loop() {
    for x in 1..11 {
        if x == 3 { continue; }

        if x == 8 { break; }

        println!("x = {}", x);
    }

    for (pos,y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_statement() {
    let country_code = 55;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        55 => "Brazil",
        1...999 => "unknown", // ... includes the last value, cannot use ..
        _ => "invalid",
    };

    println!("the country with code {} is {}", country_code, country);
}

fn main() {
    if_statement();
    while_and_loop();
    for_loop();
    match_statement();
}
