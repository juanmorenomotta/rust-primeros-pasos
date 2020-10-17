/*
* Sentencia if - else if - else
* Sentencia let-if-else
*/
fn main(){
  let x=100;   
  let y=200;

  if x>y {  
    println!("x es mayor que y"); 
  } else if x == y {
    println!("x es igual que y");
  } else {
    println!("x es menor que y");
  }

  let numero = if true{         
      100  
    } else {
      200     
    };
  
    println!("El valor de numero es {}", numero);
  
}