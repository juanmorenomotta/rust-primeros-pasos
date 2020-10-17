trait Calcular {
  fn area(&self) -> f32;
  fn perimetro(&self) -> f32;
  fn descripcion(&self) -> String;
}

struct Circulo {
  desc: String,
  radio: f32,
}

impl Calcular for Circulo {
  fn area(&self)  ->  f32 {    // implementa el metodo area()
    std::f32::consts::PI * self.radio * self.radio
  }
  
  fn perimetro(&self) -> f32 { 
    let dos: f32 = 2.0;
    std::f32::consts::PI * dos * self.radio
  } 

  fn descripcion(&self) -> String {
    self.desc.to_string()
  }


}

struct Cuadrado {
  desc: String,
  lado: f32,
}

impl Calcular for Cuadrado {
  fn area(&self) -> f32 {
    self.lado * self.lado
  }

  fn perimetro(&self) -> f32 {
    self.lado + self.lado + self.lado + self.lado
  }

  fn descripcion(&self) -> String {
    self.desc.to_string()
  }

}

fn imrimir_area<T: Calcular>(figura: T) {
  println!("Esta figura es un {} tiene un area de {}", figura.descripcion(), figura.area());
}

fn main() {
  let c = Circulo {
    desc:  String::from("Circulo"),
    radio: 1.0f32,
  };

  let s = Cuadrado {
    desc:  String::from("Cuadrado"),
    lado: 1.0f32,
  };

  imrimir_area(c);
  imrimir_area(s);
}