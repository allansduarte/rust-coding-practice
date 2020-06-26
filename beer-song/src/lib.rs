pub fn verse(n: u32) -> String {
    macro_rules! wall_status_formatter {
        () => {
            "{0} bottle{1} of beer on the wall, {2} bottle{1} of beer.\n"
        };
    };
    macro_rules! new_status_formatter {
        () => {
            "{} bottle{} of beer on the wall.\n"
        };
    };
    let wall_status;
    let action;
    let new_status;

    match n {
        0 => {
            wall_status = format!(wall_status_formatter!(), "No more", "s", "no more");
            action = String::from("Go to the store and buy some more, ");
            new_status = format!(new_status_formatter!(), 99, "s");
        }
        1 => {
            wall_status = format!(wall_status_formatter!(), n, "", n);
            action = String::from("Take it down and pass it around, ");
            new_status = format!(new_status_formatter!(), "no more", "s");
        }
        _ => {
            wall_status = format!(wall_status_formatter!(), n, "s", n);
            action = String::from("Take one down and pass it around, ");
            let mut suffix = "s";
            if n == 2 {
                suffix = "";
            }
            new_status = format!(new_status_formatter!(), n - 1, suffix);
        }
    }

    [wall_status, action, new_status].concat()
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    for number_of_bottles in (end..start + 1).rev() {
        song = [song, verse(number_of_bottles)].concat();

        if number_of_bottles != end {
            song = format!("{}{}", song, "\n");
        }
    }
    song
}
