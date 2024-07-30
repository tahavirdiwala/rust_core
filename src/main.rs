use std::any::type_name;

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


fn type_of<T>(_: T) -> &'static str { // type checker in rust
    type_name::<T>()
}

fn main() {
   let num: u32 = "42".parse().expect("Not a number !");
   println!("{}", type_of(num));
}