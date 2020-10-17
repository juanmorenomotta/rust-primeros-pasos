/*
* Gestionando Estructuras
*/
struct Rectangulo {     // creamos la estructura
  longitud: i32,
  ancho: i32,
}

fn main() {
  let tabla = Rectangulo { longitud: 10, ancho: 8 };   // inicializamos
  println!("El area del rectangulo {}", tabla.longitud * tabla.ancho);   // accedemos
}