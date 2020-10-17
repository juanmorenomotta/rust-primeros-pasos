

fn main() {
  let mut capacidad = "Capacidad de Disco Duro: 1000".to_string();
  {
    let mut mi_closure = | c: char |{capacidad.push(c)};  // closure
    mi_closure('G');   // call the closure
  }
  println!("{:?}", capacidad);   // {:?} is used to output a string
}