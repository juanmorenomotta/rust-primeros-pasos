/*
* incluir un Archivo externo
*/ 

mod ejemplo6_3;              // carga el archivo externo 
use ejemplo6_3::ex_funcion;  // carga la funcion ex_funcion 

fn main () {
  ex_funcion();      // llama a la funcion externa ex_funcion()
}