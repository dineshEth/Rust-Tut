/** ======= SINGLE DATA TYPES ======= */
fn main () {
    // * Intergers
    let _num : i8 = 10;
    let _num_1 : i16 = 250;

    // * Unsigned Integers 
    let _u_num_1 : u8 = 25;
    let _u_num_2 : u64 = 25548;

    // * Floating points
    let _float_1 : f32 = 3.25;
    let _float_2 : f64 = 6.3254;

    // * truncate   (toward zero)
    let _sum = 4 + 5;            //9
    let _sum_1 = 4.3+ 5.5;       // 9.8
    let _mult = 3*10;            // 30 
    let _devide = 10/3;          // 3

    println!("sum : {_sum}, sum : {_sum_1}, mult: {_mult}, devide: {_devide}");

    // * bool 
    let _flag = true;
    let _flag_1 : bool = false;

    // * char
    // write under single qoute 
    let _z = 'z';
    let _s : char = '😻';
    let _x : char = 'p';
}