/* 
* ownership => Pertenencia
* borrow : Prestamo
*/
fn main()  {
  let cadena = String::from("Bienvenido al mundo de Rust "); 
  let longitud = calcula_longitud(cadena);   
  println!("El valor de 'cadena' es: {}",cadena);    
  println!("La longitud de 'cadena' es: {}",longitud);  
}


fn calcula_longitud(s:String)  ->  usize { 
  s.len()    
}