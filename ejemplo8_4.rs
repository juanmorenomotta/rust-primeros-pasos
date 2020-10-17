/*
* Metodo Drop()
* “Last in, First out”.
* Cuando el método drop () se invoca automáticamente, disminuye el
* recuento de referencias, y si el número total de referencias es cero, 
* limpia los recursos en exceso asociados.
*/

struct Juego {
  numero: i32,
}
  
impl Drop for Juego {

  fn drop(&mut self) {    // define un metodo drop()
    println!("El Ganador #{}", self.numero);
  }

}

fn main() {
  let _voleyball = Juego { numero: 4 };
  let _baseball = Juego { numero: 3 };
  let _football = Juego { numero: 2 };
  let _basketball = Juego { numero: 1 };
}