/*
*  Sub modulos
*  uso de palabra clave "super"
*/

mod padre_modulo {      // modulo padre

  fn pi()  ->  f32 {        
    3.141516
  }

  pub mod hijo_modulo {      // modulo hijo
    use super::pi;           // accede a la funcion pi() del modulo padre padre_modulo 

    pub fn b() {
      println!("{}",pi());   // llama a la funcion pi() del modulo padre
    }
  }
}

fn main()  {
  padre_modulo::hijo_modulo::b();  // llama a la function b()
}