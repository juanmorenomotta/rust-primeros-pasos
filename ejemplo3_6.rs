/*
* Array y Slice
*/

fn main() {
  let mut a: [i32; 4] = [8; 4];    // creamos el array a con valores por Default
  println!("{} {} {} {}",a[0], a[1], a[2], a[3]);
  a[1] = 10;
  a[2] = 20;
  println!("{} {} {} {}",a[0], a[1], a[2], a[3]);

  let b: [f32; 6] = [ 0.10, 0.15, 0.20, 0.3, 0.4, 0.5];
  //b[1] = 0.12;
  println!("{} {} {} {} {} {}",b[0], b[1], b[2], b[3], b[4], b[5]);

  let c = &b[2..5];  // extrae de b[2] a b[4] y crea un nuevo array
  println!("nuevo array {} {} {}",c[0], c[1], c[2]);
}