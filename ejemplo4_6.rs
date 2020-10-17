/*
* Evaluación del tipo Match
*/

fn main(){

  let numero:i32 = 8;    // given expression

  match numero {
    1 => println!("uno"),
    2 => println!("dos"),
    3 => println!("tres"),    // match con este valor
    4 => println!("cuatro"),
    _default => println!("cualquier otro valor"),
  }
}