/* 
* ownership => Pertenencia
*/
fn main(){
  let x = String::from("Hola Rust Español");   // x  posee  “try”
  let y = x;                     // la pertenencia de x se ha movido a y. Cuidado!!
  println!("{}", y);             
  println!("{}", x);             // Error! x no esta disponible
}
