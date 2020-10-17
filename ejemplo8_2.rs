/*
* Trait es una Interface en Rust
*/

struct Circulo{    
  radio: f32,      
}

trait Calcular{                  // define un trait
  fn area(&self)  ->  f32;      // define un metodo trait
  fn perimetro(&self) -> f32;
}

impl Calcular for Circulo{      // implementa el trait

  fn area(&self)  ->  f32 {    // implementa el metodo Trait
    std::f32::consts::PI * self.radio * self.radio
  }
  
  fn perimetro(&self) -> f32 { 
    let dos: f32 = 2.0;
    std::f32::consts::PI * dos * self.radio
  } 

}

fn main(){
  let circulo = Circulo { radio : 2000.00};  // crea el objeto struct circulo
  println!("El area del circulo es: {}", circulo.area());  
  println!("El perimetro del circulo es: {}", circulo.perimetro());
}