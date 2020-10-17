/*
* variable @ range
*/

fn main(){
  let x = 6;

  match x {
    var @ 2 ..= 6 => println!("{}",var),    // binding
    _ => println!("otros valores"),
  }
}
