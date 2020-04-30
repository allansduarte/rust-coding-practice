#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;
use std::fmt;

struct Point {
    x:i32,
    y:i32
}

struct Line {
    start: Point,
    end: Point
}

enum Color {
    None,
    Red, 
    Green,
    Blue,    
    RgbColor(u8, u8, u8), // some kind of tuple
    CmykColor{cyan:u8, magenta:u8, yellow: u8, black: u8} // struct in stead of tuple, so it's named
}

fn main()
{
    structs();
    enumerations();
    option_of_t();
    arrays();
    vectors();
}

fn vectors()
{
    let vector_of_i32 = create_vector(3);
    print_vector_info(&vector_of_i32);

    let mut mutable_vector = create_vector(10);
    mutable_vector.push(6845);
    print_vector_info(&mutable_vector);
    let last_element = mutable_vector.pop();
    print_vector_info(&mutable_vector);
    println!("Variable last_element is an Option<T>: '{:?}'.", last_element);
    
    println!("");
    println!("Printing 'mutable_vector' values by using pop:");
    while let Some(value) = mutable_vector.pop() {
        println!(" - {}", value)
    }
}

fn arrays()
{
    // fixed length, known up front
    let mut array:[i32;5] = [1,2,3,4,5];
    let mut _array_another_way_of_declaring = [1,2,3,4,5];
    print_array_info(&array);    
    array[0] = 321;
    print_array_info(&array);

    // auto fill array
    let mut autofill = [1i32; 10];
    print_array_info(&autofill);
    // now let's fill it with a for loop from 1..10
    for i in 0..autofill.len() {
        autofill[i] = i as i32;
    }
    print_array_info(&autofill);

    // matrices
    let matrix = [
        [1i32, 0, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1]
    ];
    print_matrix_info(&matrix);
}

fn option_of_t()
{
    let _some = demonstration_of_some_or_none(3.0, 2.0);
    let _none = demonstration_of_some_or_none(3.0, 0.0);   
}

fn enumerations()
{
    match_color(Color::Red);
    match_color(Color::RgbColor(0,0,0));
    match_color(Color::RgbColor(0,0,20));
    match_color(Color::CmykColor{cyan: 0, magenta: 0, yellow: 0, black: 255});
}

fn structs()
{
    let point1 = Point { x: 10, y: 45 };
    println!("point1 (x: {} y: {})", point1.x, point1.y);

    let point2 = Point { x: 6, y: 45 };
    println!("point2 (x: {} y: {})", point2.x, point2.y);

    let line = Line { start: point1, end: point2 };
    println!("line starts at (x: {}, y: {}) and ends at (x: {}, y: {})", line.start.x, line.start.y, line.end.x, line.end.y)
}

fn match_color(color:Color)
{
    match color {
        Color::Red 
            => println!("{}", "R"),
        Color::Green 
            => println!("{}", "G"),
        Color::Blue 
            => println!("{}", "B"),
          Color::RgbColor(0,0,0) 
        | Color::CmykColor{cyan:_, magenta:_, yellow:_, black: 255} 
            => println!("Black as night!"),        
        _ => println!("{}", "?")
    };
}

fn demonstration_of_some_or_none(x: f64, y:f64) -> Option<f64>
{
    // Option<T> can either contain Some(value) or None
    let result:Option<f64> = 
        if y != 0.0 { Some(x/y) } else { None };
    println!("{:?}", result);

    match result {
        Some(value) => println!(" -> The result of the the division is '{}'.", value),
        None => println!(" -> Invalid operation (divide by zero)!!")
    }

    // destructing when some, if destructing fails then it doesn't get executed
    if let Some(value) = result { println!("    value = {}", value); }

    return result;
}

fn print_array_info(array: &[i32])
{
    // outputting information
    println!("The given array has {}# elements, with the first element containing value '{}'.", 
        array.len(),
        array[0]
    );
    println!(" -> Values: {:?}", array);
    println!(" -> Takes up {} bytes", mem::size_of_val(array));

    // playing with if
    if array.len() == 5 {
        if array != [1,2,3,4,5] { 
            println!(" -> The array has been modified!");
        } else {
            println!(" -> The array was left untouched.");
        };
    }

    // line seperator
    println!("");
}

fn print_matrix_info(matrix: &[[i32;5]; 5])
{
    // outputting information
    println!("The given matrix has {} rows.", 
        matrix.len()
    );
    println!(" -> Matrix values: {:?}", matrix);
    println!(" -> Takes up {} bytes", mem::size_of_val(matrix));

    // plauing with for loop
    println!(" -> Values from top left to bottom right:");
    for i in 0..matrix.len(){
        for j in 0..matrix[i].len() {
            if i == j {
                println!("      - [{}][{}] = {}", i, j, matrix[i][j])
            }
        }
    }

    // line seperator
    println!("");
}

fn print_vector_info<T>(vector: &Vec<T>)
    where T: 
        fmt::Debug 
      + fmt::Display
{
    // print out information
    println!("Vector contains #{} elements, the first value is '{}'.", vector.len(), vector[0]);
    println!(" -> Vector's content: {:?}", vector);

    // get an element at 999th position (doesn't exist)
    println!(" -> Trying to get value at certain position...");
    print_value_from_vector_at_position(&vector, 1);
    print_value_from_vector_at_position(&vector, 2);
    print_value_from_vector_at_position(&vector, 9999);

    // empty line
    println!("");
}

fn print_value_from_vector_at_position<T>(vector: &Vec<T>, position: usize) 
    where T: 
        fmt::Debug 
      + fmt::Display
{
    println!("    - Trying to get value at position '{}:", position);
    match vector.get(position) {
        Some(value) => println!("      - Value at index '{}' is '{}.'", position, value),
        None => println!("      - No element at index '{}'.", position)
    };
}

fn create_vector(length: i32) -> Vec<i32>
{
    let mut vector = Vec::new();

    for i in 1..(length+1) {
        vector.push(i as i32);
    }

    return vector;
}