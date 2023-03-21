// *  ======= SHADOWING OF VARIABLES ==========

fn () main{
    
    // * Shadowing
    let x = 16;
    println!("X: {x}");  // 16

    let x = x + 5;
    println!("X: {x}");   // 21

    {
        let x = x + 9 ;
        println!("X: {x}");  // 30
    }
    println!("X: {x}");   // 21

    let spaces  = "     ";  // * String 
    let spaces = spaces.len();  // * uint
    println!("spaces : {spaces}");

}