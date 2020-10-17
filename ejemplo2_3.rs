use std::io;

fn main() {
    println!("Validación de Código de Invitado");
    println!("================================");

    println!("Ingrese su código de invitado: ");

    let mut cod_invitado = String::new();

    io::stdin()
        .read_line(&mut cod_invitado)
        .expect("No puede leer!!!");

    println!("Su código de invitado es: {}", cod_invitado);
}