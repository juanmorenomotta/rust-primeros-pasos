/*
* Error checking
* assert! (variable == true/false)
*/


fn main()  {   
  let check : bool = true;    // suponemos que es verdadero
  assert!(check == true);   // validamos el error
  println!("{}", check);
}