/*
* Ambito de las funciones
*/

fn f() { println!("1"); }

fn main() {
    f(); //Imprime 2

    {
        f(); // Imprime 3
        fn f() { println!("3"); }
    }

    f(); // Imprime 2

    fn f() { println!("2"); }
}