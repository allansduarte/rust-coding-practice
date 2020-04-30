extern crate rand;
extern crate phrases;
use phrases::greetings::french;
use rand::Rng;

fn main() {
    println!("English: {}, {}", 
      phrases::greetings::english::hello(), 
      phrases::greetings::english::goodbye()
    );
    println!("French: {}, {}", 
      french::hello(), 
      french::goodbye()
    );

    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}