
fn main() {
    //obtener nombre del usuario
   println!("Introduce tu nombre:");
   let mut nombre : String = String::new();
   std::io::stdin().read_line(&mut nombre).unwrap();
   nombre = nombre.trim().to_string();
   // obtener edad del usuario
   println!("Introduce tu edad:");
   let mut edad : String = String::new();
   std::io::stdin().read_line(&mut edad).unwrap();
   edad = edad.trim().to_string();
   // obtener nacionalidad del usuario
   println!("Introduce tu nacionalidad:");
   let mut nacionalidad : String = String::new();
   std::io::stdin().read_line(&mut nacionalidad).unwrap();
   nacionalidad = nacionalidad.trim().to_string();


   println!("Hola bienvenido {} con {} de {}", nombre, edad, nacionalidad);
}
