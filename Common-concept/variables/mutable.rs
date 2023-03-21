//* ======== MUTABLE VARIABLES ==========

fn main(){
    // ** Mutable variables
    // * ===>  "mut"  keyword to make a variable muttable
    // * below varaibles are immutable,
    // * it means their value can be assign
    let mut id = 151024;
    let mut user_name = "simon41";
    let mut favourite =  "#";

    println!("id: {id}, userName: {user_name}, favourite: {favourite}");
    // * resign values tp varaibles
    id=1513;
    user_name = "jsoes";
    favourite = "%";
    println!("id: {id}, userName: {user_name}, favourite: {favourite}");

    // Important "You cannot change the type of the variable while re-assigning"
    // let mut str = "veer";  // * String
    // str = str.len();       // * uint

}