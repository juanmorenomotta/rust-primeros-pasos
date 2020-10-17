/*
* Sentendia loop{ }
*/
fn main(){

  let mut i: i8 = 2;

  loop{  // iniciamos el bucle infinito
    println!("Aprende Rust en {} horas", i );    
    if i == 6 {  // realizamos una evaluación condicional    
      break;     // rompemos el bucle
    }    
    i = i + 1;
  }

}