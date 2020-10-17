/*
* Match
* Patron Range
* number1..=number2
* Viene del ejemplo4_6.rs
*/ 
use std::io;

fn main(){

  let mut numero: i8;
  let mut cadena = String::new();
  
  loop {

    println!("Ingrese un numero entero entre 1 y 10");
  
    io::stdin()
        .read_line(&mut cadena)
        .expect("No puede leer cadena!!!");

    numero = cadena
      .trim()
      .parse()
      .expect("Se requiere un número entero");
    
    cadena = "".to_string(); 
    
    if numero == 0 || numero > 10 {  // realizamos una evaluación condicional    
      break;     // rompemos el bucle
    }   

    match numero {

      2 ..= 6 => {
          println!("desde el valor 2 hasta el valor 6");   
        } 
      _ => println!("otros valores"),

    }

  }

}

