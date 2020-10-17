/*
* Definición y ejecución de un modulo embebido
*/
mod mod1{ 

  pub fn a(){
    println!("Funcion a() del modulo mod1");
  }

  pub mod mod2{       
    pub fn b(){
      println!("Funcion b() del modulo mod2");
    }
  }

}

fn main(){
  mod1::a();
  mod1::mod2::b();   
}