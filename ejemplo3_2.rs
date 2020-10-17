// Asignación de tipo String
fn main(){
  let x = "Hola Mundo Rust".to_string();
  let y = String::from("Hola Mundo Rust");
  let z:&str = "Hola Mundo Rust";
  print!("{} ; {} ; {} ", x, y, z);
  println!();
  
  let cadena = String::from("inmensamente");
  let longitud = calcula_longitud(&cadena);
  println!("La longitud de '{}' es {}.", cadena, longitud);
}

fn calcula_longitud(s: &String) -> usize {
  s.len()
}