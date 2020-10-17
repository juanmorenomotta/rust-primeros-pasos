/*
*  Modulos con llamadas a funciones privadas en su propio ambito
*/

mod mi_modulo  {

  pub fn a()  {    // dcaramos function a() como publica
    println!("function publica a()");    
    b();    
  }

  fn b(){         // por default la function b() es privada
    println!("function privada b()");
  }

}

fn main()  {
  mi_modulo::a();  
}