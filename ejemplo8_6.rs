/*
* Closure
*/

fn main() {
  let mut capacidad = "Capacidad de Disco Duro: 1000".to_string();
  {
    let mut mi_closure = | c: char |{capacidad.push(c)};  // crea un bloque de codigo
    mi_closure('G');   // llama al bloque de codigo "mi_closure()"
  }
  println!("{:?}", capacidad);   // {:?} usado para mostrar una cadena de caracteres
}