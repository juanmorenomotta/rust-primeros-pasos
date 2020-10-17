/*
* Gestionando Estructuras
*/

struct Usuario {   // creamos la estructura
  id: i32,                   // miembro: tipo
  nombre: String,
  apellido: String,
  vacaciones: bool,
}

fn main() {

  let usuario = Usuario {   // inicializa la estructura
    id: 016320,                   // miembro: valor
    nombre: "Juan".to_string(),
    apellido: "Moreno".to_string(),
    vacaciones: false,
  };

  println!("------------------------");
  println!("ID         : {}", usuario.id);   // access the members
  println!("Nombre     : {}", usuario.nombre);
  println!("Apellido   : {}", usuario.apellido);
  println!("Vacaciones : {}", usuario.vacaciones);
  println!("------------------------");

}