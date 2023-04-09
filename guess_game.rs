use std::io;   // lib for I/O

fn main(){

let mut guess = String::new(); // new string
std::io::stdin().read_line(&mut guess);  //reading into guess ,passed as reference, so ownership is not passed.
println!("{}",guess);

}