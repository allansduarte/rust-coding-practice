struct Drop {
    id: u32,
    sound: String,
}

impl Drop {
    /// Creates a new Drop.
    fn new(id: u32, sound: &str) -> Drop {
        Drop {
            id: id,
            sound: sound.to_string(),
        }
    }
}

pub fn raindrops(n: u32) -> String {
    let drops = vec![Drop::new(3, "Pling"), Drop::new(5, "Plang"), Drop::new(7, "Plong")];
    let mut sounds: String = "".to_owned();

    for drop in drops.iter() {
        if n % drop.id == 0 {
            sounds.push_str(&drop.sound);
        }
    }

    if sounds != "" {
        return sounds;
    } else {
        return n.to_string();
    }
}
