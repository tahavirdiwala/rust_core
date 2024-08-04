// use std::any::type_name;

// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;                              // you cannot assign variable twice because by default rust variables are immutable
//     println!("The value of x is: {x}");
// }

//shadowing
// diff between shadowing and mut is shadowing can change the type after new declaration with let keyword;

// fn main() {
//     let x = 5;

//     let x = x+1;

//     {
//         let x = x*2;
//         println!("innerScope : {}", x)
//     }

//     println!("parentScope : {}", x)
// }

// fn main() {
//     let mut spaces = "   ";
//     spaces = spaces.len();
//     println!("spaces: {}", spaces)
// }


// fn type_of<T>(_: T) -> &'static str { // type checker in rust
//     type_name::<T>()
// }

// fn main() {
//    let num: u32 = "42".parse().expect("Not a number !");
//    println!("{}", type_of(num));
// }

// tuple 

// fn main() {
//     let tup = (500, 6.4, 1);

//     // let (x, y, z) = tup;

//     // println!("x: {}", x);
//     // println!("y: {}", y);
//     // println!("z: {}", z)

//     let x = tup.0;

//     println!("{}", x)
// }

// arrays

// fn main() {
//     let list = [1, 3, 4, 17, 81];

   
//     for (index, item) in list.iter().enumerate() {
//         println!("{} and {}", index, item)
//     }


//     for (index, _) in list.iter().enumerate() {
//         println!("list: {}", list[index])
//     }

// }

// fn main() {
//     //array slicing
//     let arr = [1,2,3,4,5];

//     for val in middle.iter() {
//         println!("{}", val);
//     }

// }

// use std::io;

// fn main() {
//     let arr = [1,2,3,4,5];

//     let mut index = String::new();

//     println!("Please enter an array index.");

//     io::stdin()
//     .read_line(&mut index)
//     .expect("Falied  to read line");

//     let index: usize = index.trim().parse().expect("Index entered is not valid");

//     let answer = arr[index];

//     println!("ans : {}", answer)

// }

// fn main() {
//    let x = plus_one(5);

//    println!("the value is : {}", x)
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }


// if/else

// fn main() {

//     // if num % 4 == 0 {
//     //     println!("divisible by 4");
//     // } else if num % 3 == 0 {
//     //     println!("divisible by 3");
//     // } else if num % 2 == 0 {
//     //     println!("divisble by 2")
//     // } else {
//     //     println!("number is not divisible by 4, 3, or 2")
//     // }


//     let predicate = true;

//     let number = if predicate { 5 } else { 6 };
//     println!("{number}")
// }

// loops

//loop

// fn main() {
//     let mut count = 0;

//     let result = loop {
//         count += 1;

//         if count == 10 {
//             break count * 2;
//         }
//     };

//     println!("{}", result);
// }

// fn main() {
//     let mut count = 0;

//     'counting_up: loop {
//         println!("{}", count);

//         let mut remaining = 10;

//         loop { 
//             println!("{}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;

//     }
// }

//while loop 

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// fn main() {
//     let a = [1,2,3,4,5];

//     for item in a.iter().rev() {
//         println!("{}", item)
//     }
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let obj = build_user(String::from("some@gmail.com"), String::from("shanks"));
//     println!("{:#?}", obj);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// struct Rectangle {
//     width: u32,
//     height: u32
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(dimensions: &Rectangle) -> u32 {
//     dimensions.width * dimensions.height
// }

//methods
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height:50
    };

    println!("{}", rect.area())
}