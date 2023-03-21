
//*  All the functions are created outside the main function
//* all function are called inside main functions
//* Functions which are called inside main function are compiled and  executed by compiler

fn main(){
    let y = five();
    println!("y : {y}");
    greet();
    number(5);
    print_detail(10,'%');
}

// ** Non-Parameter function
fn greet () {
    println!("Good morning");
}

//**  Parameter function 
fn number (x : i32){
    println!("The value of X is : {x}");
}

fn print_detail (x: i32, y: char) {
    println!("The char : {y} is associated with x : {x}");
}


//* Functions with return values
fn five () -> i32 {
    5
}

fn sum (x : i32, y: i32) -> {
    // x + y;   throws error
    x + y
}

