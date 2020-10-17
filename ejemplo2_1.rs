fn main(){
  let numero = 88;
  let var = "OK";
  let entero: i32 = -100;
  let entero_sin_signo: u32 = 45;
  let cadena: String = "Hola Mundo".to_string();

  const FLOTANTE: f32 = 100.88;
  const ENTERO32: i32 = FLOTANTE as i32;

  println!("El valos de var es {} y el valor de numero es {}", var, numero);
  println!("El valos de entero es {}", entero);
  println!("El valos de entero_sin_signo es {}", entero_sin_signo);
  println!("El valos de cadena es {}", cadena);
  println!("El valos de FLOTANTE es {}", FLOTANTE);
  println!("El valos de ENTERO32 es {}", ENTERO32);
}