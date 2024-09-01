//! cargo new rust_eval
//!
//! cargo build
//! <br>
//! cargo run --release
//! <br>
//! cargo clean
//!
//! cargo doc --open
//! <br>
//! rustup doc --book
//!

use std::i32;

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    let x = 0;

    let message = if 1 > x {
        "less"
    } else if 1 == x {
        "equal"
    } else {
        "greater"
    };
    println!("The message is: {}", message);
    // let (tup1, tup2, tup3) = tup;

    let mut tup = (1, 2, 3);

    println!("meow, {tup:?}",);
    tup.1 = 11;

    println!("After mutation{:?}", tup);

    struct Point {
        x: i64,
        y: i64,
        z: i64,
    }

    let point = Point { x: 1, y: 2, z: 3 };
    let x = point.x;
    let Point { x, y, z } = point;
    let Point { y, .. } = Point {
        x: 11,
        y: 22,
        z: 33,
    };

    println!("Struct Point, {:?}", (x, y, z));

    let _years: [i32; 3] = [1995, 2000, 2005];

    let _array1 = [123____u64, 321, 333];
    let array2 = [244_u8; 3];

    println!("Array2 = {:?}", array2);

    //? Frontend-Masters told that we cant assign values to enums
    enum Color {
        Green,
        Yellow,
        Red,
        Custom { red: u8, green: u8, blue: u8 },
    }

    //* Self = Color
    //* self = color:Color
    impl Color {
        fn rgb(self) -> (u8, u8, u8) {
            // match the Green Red or Yellow with rgb(u8,u8,u8)
            return (0, 255, 0);
        }
        fn new(r: u8, g: u8, b: u8) -> Self {
            // match the values with the Colors
            return Self::Green;
        }
    }

    // arr with 14 elements filled with Zeroes with type of f32
    let array_float = [0_____f32; 14];

    let go = Color::Green;
    let steady = Color::Yellow;
    let stop = Color::Red;
    let blurple = Color::Custom {
        red: 88,
        green: 101,
        blue: 242,
    };

    let can_go: Color = Color::Yellow;
    match can_go {
        Color::Green => {
            println!("Of course! It's Green!");
        }
        Color::Yellow => {
            println!("You'd better wait a bit! It's Yellow!");
        }
        Color::Red => {
            println!("NO, don't even try! It's Red!");
        }
        Color::Custom { red, green, blue } => {
            println!("Custom match: {}, {}, {}", red, green, blue);
        } // _ => {
          //     println!("It was Match All Pattern with _");
          // }
    }

    let mut vec1 = vec![1995, 2000, 2005];
    vec1.push(2010);
    println!("Capacity = {}; Length = {}", vec1.capacity(), vec1.len());

    let vec2 = vec![30; 2];
    println!("vec2 = {:?}", vec2);

    //? Frontend-Masters told that usize bits depends on system bits. And noticed that wasm uses x32
    let max_usize = usize::MAX;

    let mut vec3: Vec<i32> = Vec::with_capacity(2);
    println!("Array2: {:?}", vec3.push(44));

    vec2.len();

    let some_str = "Some string";

    let word_string = &some_str[5..some_str.len()];

    println!(
        "My sub-slice is equal to: {}. With a size of {}",
        word_string,
        word_string.len()
    );

    let arr_slice = &array2[1..2];

    print_indexes(&some_str);

    let mut vec4: Vec<i64> = Vec::with_capacity(5);
    println!("Print vec4 slice {:?}", &vec4);
    vec4 = vec![1, 2, 3, 4];
    println!("Print vec4 slice {:?}", &vec4);

    //? Static cant be accessed out of scope. But afaik the memory of the static is not cleared till finish
    {
        let static_num: &'static i32 = &15;
    }

    // Means &'static str. And the "Name" string will only be deallocated on the program end
    // &'static variables are just references to data built in binary
    let name: &str = "Name";

    // Lifetime Annotations - You can not create a struct/enums referencing some data without naming the lifetime
    // struct Foo<'a> { x: &'a[i64] }

    square::print_square();
    square::print_hollow_square();

    
}

#[allow(dead_code)]
// Looks like its better to use slice `&[i64]` parameter instead of Vec<>
fn sum(numbers: Vec<i64>) -> i64 {
    let mut total = 0;

    for num in numbers {
        total += num;
    }
    total
}

fn print_indexes(string: &str) {
    let mut count: usize = 0;

    let mut ch: char;

    for i in string.chars() {
        ch = match count {
            0..10 => '0',
            _ => '\0',
        };

        println!("{ch}{count}, {i}");
        count += 1;
    }
}

mod square {
    const SQ_SYMBOL: &str = "+";
    const SQ_SPACE: &str = " ";
    const SQ_SIZE: i32 = 6;

    pub fn print_square() {
        println!("Start of the Square!");
        for _ in 1..=SQ_SIZE {
            for _ in 1..=SQ_SIZE {
                print!("{}", SQ_SYMBOL);
            }
            println!();
        }
    }

    pub fn print_hollow_square() {
        println!("Start of the Hollow Square!");
        for x in 1..=SQ_SIZE {
            for y in 1..=SQ_SIZE {
                print!(
                    "{}",
                    match x {
                        1 => SQ_SYMBOL,
                        2..SQ_SIZE => {
                            if y == 1 || y == SQ_SIZE {
                                SQ_SYMBOL
                            } else {
                                SQ_SPACE
                            }
                        }
                        SQ_SIZE => SQ_SYMBOL,
                        _ => todo!(), // To suppress the warning also valid i32::MIN..=i32::MAX
                    }
                );
            }
            println!();
        }
    }
}
