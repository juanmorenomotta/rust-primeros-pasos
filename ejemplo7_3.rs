/*
* Vectores: Tercera forma de crear vectores y asignar valores
* let mut v = Vec::new(); 
* v.push('value');  
*/


fn main() {
  let mut vector=Vec::new(); // creamos el vector
  vector.push('R');    // asignamos un primer elemento al vector  
  vector.push('U');  
  vector.push('S');
  vector.push('T');  

  for n in vector {   
    print!("{}",n);  
  }
  println!();
}