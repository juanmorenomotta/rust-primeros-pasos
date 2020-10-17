/*
* enum
*/
enum Lenguaje{    // definimos un enum
  RUST,      // miembro
  GO,
  CSHARP,
  C,
  CPP,
  JAVA,
  PHP,
}

fn programa( parametro: Lenguaje ){   // recibimos un parametro de tipo Lenguaje
  match parametro{     
    Lenguaje::RUST   => println!("RUST puesto 18 en TIOBE"),    
    Lenguaje::GO     => println!("GO   puesto 14 en TIOBE"),    
    Lenguaje::CSHARP => println!("C#   puesto  5 en TIOBE"),  
    Lenguaje::C      => println!("C    puesto  1 en TIOBE"),  
    Lenguaje::CPP    => println!("CPP  puesto  4 en TIOBE"),  
    Lenguaje::JAVA   => println!("JAVA puesto  2 en TIOBE"),  
    Lenguaje::PHP    => println!("PHP  puesto  8 en TIOBE"),  
  }
}


fn main(){
  programa(Lenguaje::RUST);    
  programa(Lenguaje::GO);
  programa(Lenguaje::CPP);
  programa(Lenguaje::C);
  programa(Lenguaje::PHP);
  programa(Lenguaje::JAVA);
  programa(Lenguaje::CSHARP);
}