pub fn var(){
    // let x = 12; // immutable
    // x = 14; !cant assign twice
    // lp(x);
/*
    let mut y = 15;
    y = 56;
    lp(y);

    let a = 23;
    b = a.clone();
*/

    let x = 12;
    lp(x);

    

    
    
}


pub fn lp(var: i32){
    println!("{}", var);
}

pub fn arguments(){
    
    println!("The {} {} thing was amazing", "New", "Horizon");
    
    // positional arguments
    println!("The {0} {1} thing was amazing", "New", "Horizon");

    // named arguments
    println!("{user} has a {mem} membership", user= "Rakib", mem= "premium");

    // placeholders traits
    let num = 12;
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", num, num, num);
    
    // placeholders for debug traits
    println!("{:?}", ("premium", num, "rakib")); // tuple

}