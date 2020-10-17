/*
* Definición y ejecución de un modulo
*/
mod mi_modulo{       // define el modulo

  pub fn prueba(){     // define como publico el atributo function prueba
    println!("Estas llamanando a mi_modulo::prueba()");
  }

}

fn main(){
  mi_modulo::prueba(); // ejecuta el atributo prueba del modulo mi_modulo 
}