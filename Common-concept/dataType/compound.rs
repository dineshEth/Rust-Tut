/** ========== COMPOUND DATA TYPE =========== */

fn main() {
    //* tuple 
    let tup : (i32,f32,bool) = (500, 1.5, true);
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("a: {a}, c: {b}, c: {c}");

    let tup1 = (500, 600, 900);
    let (x,y,z) = tup1;
    println!("x: {x}, y: {y}, z: {z}");


    // * array 
    let a = [0, 1, 2, 3, 4, 5, 6];
    let days = ["Mon","Tue","Wed","Thur","Sat","Sun"];

    println!("3rd day :{} and place : {}",days[2],a[5]);

    let b : [i8,3] = [1,2,3];  // datatype and length
}