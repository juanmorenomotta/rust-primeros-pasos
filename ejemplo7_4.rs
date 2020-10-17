/*
* Match
* Patron multiple valor 
* valor1 | valor2
* Viene del ejemplo4_6.rs
*/ 

use std::io;

fn main(){

  let numero: i8;
  let mut cadena = String::new();

  println!("Ingrese un numero entero entre 1 y 10");
  
  io::stdin()
      .read_line(&mut cadena)
      .expect("No puede leer cadena!!!");

  numero = cadena
    .trim()
    .parse()
    .expect("Se requiere un número entero");

  match numero {
  1 => println!("evalua uno"),
  2 | 3 => println!("evalua dos y tres"),   // patron de evaluación multiple
  _ => println!("otros valores"),
  }

}
