/*
* Match
* Patron Range
* number1...number2
* Viene del ejemplo4_6.rs
*/ 

fn main(){

  let x = 3;

  match x {

    2 ... 6 => {
        println!("desde el valor 2 hasta el valor 6");   
      } 
    _ => println!("others"),

  }
}

