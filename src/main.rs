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