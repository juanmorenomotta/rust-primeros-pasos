/*
* Match
* Patron multiple valor 
* valor1 | valor2
* Viene del ejemplo4_6.rs
*/ 

fn main(){
  let numero: i8 = 3;

  match numero {
  1 => println!("evalua uno"),
  2 | 3 => println!("evalua dos y tres"),   // patron de evaluación multiple
  _ => println!("otros valores"),
  }

}
