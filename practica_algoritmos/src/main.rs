fn main () {
    // let n: i32 = algoritmo_1(5,10);
    // println!("{}",n);
    // let arr:[i32;5] = [1,3,5,7,9];
    let arr_2:[i32;9] = [10,7,5,12,20,2,8,13,6];
    println!("{:?}",arr_2);
    // algoritmo_1(arr,arr_2);
    algoritmo_burbuja(arr_2);
    // println!("{:?}",arr_3);
    
}
// fn algoritmo_1(mut dato: [i32;5],dato_2: [i32;5]) {
//     for i in 0..dato.len(){
//         dato[i] +=dato_2[i];
//         println!("{}",dato[i]);   
//     }
// }
fn algoritmo_burbuja(mut dato: [i32;9]){
    let ultimo = dato.len();
    for i in 0..ultimo{
        if i < ultimo-1 && dato[i] > dato[i+1]{
            let u = dato[i+1];
            dato[i + 1] = dato[i];
            dato[i] = u;
        }
    }
    println!("{:?}",dato);
}
