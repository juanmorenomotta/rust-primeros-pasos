/*
* Trait con tipo Generico
*/

pub trait Show {    // define un trait
  fn show(&self);      // define un metodo
  fn lenght(&self);
}

impl<T> Show for T        // implementando el trait con objeto tipo generico
  where T: ToString {     // especifica el tipo String

  fn show (&self){        // implementa el metodo show()
    println!("{}",self.to_string());
  }

  fn lenght (&self) {
    println!("{}", self.to_string().len());
  }
}
  
fn main(){
  String::from("Bienvenido al mundo de Rust").show();     // llama al metodo show()
  String::from("Bienvenido al mundo de Rust").lenght();     // llama al metodo show()
  "hello".to_string().lenght();
}