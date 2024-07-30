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

