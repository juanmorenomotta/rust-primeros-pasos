/*
* Una funcion dentro de un Struc, Trait, Enum es denominado metodo
*
*  impl Struct/Enum {    // implementa un Struct o Enum
*     fn method_name(&self)  ->  type {   // define un metodo
*         self.member    // accede a un miembro o atributo 
*     }
*  }
*/

struct Circulo{      // define/crea un tipo struct
  radio: f32,        // miembro/atributo de la struct
}

impl Circulo{        // implementa la estructura

  fn area(&self)  ->  f32 {    // define el metodo area()
    std::f32::consts::PI * self.radio * self.radio
  }
  
  fn perimetro(&self) -> f32 { // define el metodo perimetro()
    //let dos: f32 = 2.0;
    std::f32::consts::PI * 2.0f32 * self.radio
  }

}

fn main(){

  let circulo = Circulo { radio : 2000.00 };  // crea el objeto struct
  println!("El area del circulo es: {}", circulo.area());  
  println!("El perimetro del circulo es: {}", circulo.perimetro());  

}

