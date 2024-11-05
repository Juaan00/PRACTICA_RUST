fn main () {
    // let n: i32 = algoritmo_1(5,10);
    // println!("{}",n);
    let arr:[i32;5] = [1,3,5,7,9];
    let arr_2:[i32;5] = [10,9,8,7,6];
    algoritmo_1(arr,arr_2);
}
fn algoritmo_1(mut dato: [i32;5],dato_2: [i32;5]) {
    for i in 0..dato.len(){
        dato[i] +=dato_2[i];
        println!("{}",dato[i]);   
    }

fn algoritmo_2(dato:[i32;10],objetivo:i32){

}
}