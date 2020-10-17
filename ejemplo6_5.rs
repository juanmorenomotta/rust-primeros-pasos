/*
*  Modulos con funciones privadas
*/

mod mi_modulo  {
  pub fn a()  {    // declaramos function a() como publica
    println!("function publica a()");        
  }
  fn b(){         // por default la function b() es privada
    println!("function privada b()");
  }
}

fn main()  {
  mi_modulo::a();
  mi_modulo::b();   // llamada a la function privada b()
}