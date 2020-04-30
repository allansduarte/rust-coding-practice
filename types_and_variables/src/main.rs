const PI_SHORTHAND:f64 = 3.14; // this constant no fixed address

static MEANING_OF_LIFE:i8 = 42; // static is another way of a global value
static mut MUTABLE_GLOBAL_VARIABLE:i8 = 24;

mod stack_and_heap;

fn main() {
    data_types();
    operators();
    scope_and_shadowing();
    contants();
    stack_and_heap();
}

fn stack_and_heap()
{
    let _x = 5; // i32
    // this is placed on the stack, 
    //   - short term storage
    //   - faster, but limited space

    let y = Box::new(10);
    // this is placed on the heap,
    //   - long term storage
    //   - y is on the stack, but it contains the address of the location in memory where the value of y is located
    //     aka y holds a reference    
    println!("y's value = {}", *y);


    // stack and heap demonstration in other module
    stack_and_heap::stack_and_heap();
}

fn contants()
{
    println!("Pi in short form is '{}'", PI_SHORTHAND);
    println!("Meaning of life is '{}'", MEANING_OF_LIFE);

    unsafe {
        println!("MUTABLE GLOBAL VARIABLE '{}'", MUTABLE_GLOBAL_VARIABLE);
        MUTABLE_GLOBAL_VARIABLE = 0; // marking it as unsafe, we can of course then change a mutable globally declared variable
        println!("MUTABLE GLOBAL VARIABLE '{}'", MUTABLE_GLOBAL_VARIABLE);
    }
}

fn scope_and_shadowing()
{
    let a = 5;
    println!("a = {}", a); // a is in scope of function main, local scope.

    some_function();

    {
        let b = 456;
        println!("b = {}", b);

        println!("a = {}", a); // but a does exists in this inner scope

        let a = 123123; // but we can declare another a in the inner scope
        println!("a = {}", a); // so we can "redeclare" the variable
    }

    // here b is not available...
    // println!("b = {}", b); // this won't compile!

    // the outer a is still 5
    println!("a = {}", a);
}

fn operators()
{
    // arithmethic
    let mut a = 2+3*4/2; // +-*/
    println!("{} | result of 2+3*4/2 should be '{}', acutal value is '{}'.", 8 == a, 8, a);
    
    a += 1;    
    println!("{} | result of a += 1 should be '{}', actual value is '{}'.", 9 == a, 9, a);


    // modulus    
    println!("{} | result of 13%2 should be '{}', actual value is '{}'.", 13%2 == 1, 1, 13%2);


    // power of x
    let n = 5;
    let power_of_result = i32::pow(n, 2);
    println!("{} | result of 5² should be '{}', actual value is '{}'.", 25 == power_of_result, 25, power_of_result);

    let b = 2.5;
    let b_cubed = f64::powi(b, 2);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("b to the power of 2 => {}", b_cubed);
    println!("b to the power of 3.14 => {}", b_to_pi);


    // bitwise
    let c = 1 | 2; // | OR
                   // & AND
                   // ^ XOR
                   // ! NOR
    println!("{}", c);


    // shift operators
    let two_to_10 = 1 << 10;
    println!("2 to the power of ten (with shift to the left) == {}", two_to_10);


    // logical operators
    let pi_less_than_four = std::f64::consts::PI < 4.0;
    println!("{}", pi_less_than_four);
    // equality is done by '==', inequality is '!='
}

fn data_types()
{
    use std::mem;

    let a:u8 = 123; // 8bits
    println!("a = {}", a);

    let mut b:i8 = 0; // mutable
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; //32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification", c);

    // i8 u8 i16 i32 u32 i64 u64
    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e:f32 = 2.5; // double-precision, 8 bytes or 64 bits, f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // true or false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));

    let f = 4 > 0;
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));
}

fn some_function(){
    // a doesn't exist here
    // println!("a = {}", a); // this won't compile!
}