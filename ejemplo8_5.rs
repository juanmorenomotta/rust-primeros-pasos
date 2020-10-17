/*
* Closure: Es una función anónima en Rust
* (1) Create a closure
* let closure_name = | parameter | {  };
* (2) Call the closure
* closure_name(parameter);
*/

fn main(){
  let mi_closure = | numero: i32 | { numero + 200 };  // crea un closure
  let numero = 100;    
  println!("{}", mi_closure(numero));    // llama al closure
}

