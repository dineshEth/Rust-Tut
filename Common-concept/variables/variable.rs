/** ================== VARIBALES IN RUST ================ */


fn main() {
    // * Immutable vaiables (default) 
    // * below variable are immutable,
    // * now their value cannot be change 
    let num = 12;     
    let name = "Veer";
    let symbol = "V"; 

    // * throw error  
    // num = 15;
    // name = "Joe";
    // symbol = "j"; 

    println!("Number: {num} , Name: {name}, Symbol: {symbol}"); 


    // * constant variable which are not changed throughput the program
    const PI : f64 = 3.14;     // you have to define the type of the variable 
    println!("PI = {PI}");


}