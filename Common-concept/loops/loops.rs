//**  LOOPS (repetion of code)
//*  loop , while , for

fn main () {
    //*** loop is infinite loop
    // loop {
    //    println!("Hello");
    // }

    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 3 {
            break counter * 2;
        }
    };
    println!("The result is {result}");



    //*** while
    let mut number = 3 ;
    
    while number != 0 {
        println!("{number}!");
        
        number -=1;
    }
    println!("LIFTOFF!");


    // *** for... in
    let a = [1,2,3,4,5];

    for element in a {
        println!("the value is : {element}");
    }

    for number in (1..4).rev(){
        println!("{number}");
    }
}