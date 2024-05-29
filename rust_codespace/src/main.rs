mod modulo;
fn main() {
        println!("Bienvenido al registro de datos de estudiantes, tenemos en el registro de datos de estudiantes lo siguiente:");
        
        let estudiante = modulo::primer_estudiante();
        let estudiante2 = modulo::segundo_estudiante();
        let estudiante3 = modulo::tercer_estudiante();
        let grupo = vec![estudiante,estudiante2,estudiante3];
        for estudiante in grupo.iter(){
                println!("{}",estudiante);
        }
}