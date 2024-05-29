use std::fmt;
enum Activo {
    A,
    I
}
pub struct Estudiante{
    código: i32,
    nombre_apellido: String,
    correo: String,
    activo: bool,
    materias: Vec<String>,
}
impl fmt::Display for Estudiante{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            let x = "_";
            println!("{}",x.repeat(120));
            write!(f, "\nNombre - {}
            \nCódigo - {}
            \nCorreo - {}
            \nActivo - {}
            \nMaterias:\n",self.nombre_apellido,self.código,self.correo,self.activo)?;
            let mut i = 1;
            for materia in self.materias.iter(){
                    write!(f, "\n     {} - {}.",i,materia)?;
                    i = i + 1;
            }
            Ok(())
    }
}
pub fn primer_estudiante() -> Estudiante {

    let activo:Activo = Activo::A;

    let estudiante1 = Estudiante{
        código: 1136887617,
        nombre_apellido: String::from("Juan Felipe Quiroga"),
        correo: String::from("juquirogam@unal.edu.co"),
        activo:match activo  {
            Activo::A =>true,
            Activo::I =>false
        },
        materias: vec![
            String::from("Programación de Computadores"),
            String::from("Cálculo Diferencial"),
            String::from("Pensamiento Sistémico"),
            String::from("Introducción a la Ingeniería")],
    };
    estudiante1
}
pub fn segundo_estudiante() -> Estudiante {
    let activo:Activo = Activo::I;
    let estudiante2 = Estudiante{
        código: 1234567890,
        nombre_apellido: String::from("Máximo Décimo Meridio"),
        correo: String::from("maxi_deci@unal.edu.co"),
        activo: match activo {
            Activo::A =>true,
            Activo::I =>false
        },
        materias: vec![
            String::from("Programación de Computadores"),
            String::from("Cálculo Diferencial"),
            String::from("Pensamiento Sistémico"),
            String::from("Introducción a la Ingeniería")],
};
    estudiante2
}
pub fn tercer_estudiante() -> Estudiante{
    let activo:Activo = Activo::A;
    let estudiante3 = Estudiante{
        código: 0987654321,
        nombre_apellido: String::from("Emma Demonioson"),
        correo: String::from("emma@unal.edu.co"),
        activo: match activo {
            Activo::A => true,
            Activo::I => false
        },
        materias: vec![
            String::from("Programación de Computadores"),
            String::from("Cálculo Diferencial"),
            String::from("Pensamiento Sistémico"),
            String::from("Introducción a la Ingeniería")],
        
    };
    estudiante3
}