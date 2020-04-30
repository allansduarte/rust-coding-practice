fn main()
{
    if_statement();
    while_and_loop();
    match_statement();
}

fn match_statement()
{
    let country_code = 32; // 1 to 9999

    let country = match country_code {
        32 => "Belgium",
        44 => "United Kingdom",
        7 => "Russia",
        1..=31 => "Somewhere", // 1..31 it would not include 31, but 1....31 will include 31
        _ => "unknown"
    };

    println!("Code '{}' corresponds to '{}'.", country_code, country);
}

fn while_and_loop()
{
    let mut x = 1;
    
    println!("while");
    // while loop
    while x < 1000 {
        x *= 2;
        println!("x = {}", x);
    }

    println!("");
    println!("while (with continue;)");
    // while loop with continue instruction
    x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 { continue; } // this skips the value 64, skips everything below
        println!("x = {}", x);
    }

    println!("");
    println!("loop (with break;)");
    // loop
    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 { break; } // breaks out of the loop, can also be used in the while loop
    }

    println!("");
    println!("for loop");
    for x in 1..11 { //11 is the upper bound
        println!("x = {}", x);
    }

    println!("");
    println!("for loop (position, and element)");
    for (pos,x) in (30..41).enumerate() {
        println!("{}: is = {}", pos, x);
    }
}

fn if_statement()
{
    let temp = 35;

    // basic if flow
    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10 {
        println!("really cold!");
    } else {
        println!("temperature is ok!");
    }

    // if as an expression
    let day = if temp > 20 {"sunny"} else {"cloudy"}; // the if is actually an expression in rust!
    println!("today is {}", day);
    println!(
        "it is {}", 
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"ok"}
    );

    // if inside if's (nested)
    println!(
        "it is {}",         
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"ok"}
    );
}