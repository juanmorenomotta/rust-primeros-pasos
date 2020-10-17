/*
* Funciones: Con y Sin retorno de datos
*/


fn main(){
  suma(100, 200);

  let numero = incrementa(100);
  println!("El valor de numero es : {}", numero);

  let estado: bool = cancela();
  println!("El estado actual es: {}", estado); 
}

fn suma(x:i32, y:i32){
  println!("La suma de x + y es : { }", x+y);
}

fn incrementa(numero: i32) -> i64 {
  let nuevo: i32 = numero + 50;
  return nuevo as i64
}

fn cancela() -> bool {
  return false
}