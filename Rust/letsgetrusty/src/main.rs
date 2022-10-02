fn main() {

    println!("============ LetsGetRusty =============");
//    variables();
    functions();

}

fn variables(){
    fn constants(){
    //    const PI:f32 = 3.1415;
        const PI:f32 = std::f32::consts::PI;
        println!("the value of pi is {}", PI);
    }
//    constants();

    fn mutable_variables() {
        let mut x = 23;
        println!("the present value of x is {}", x);
        x = 34;
        println!("after changed value of x is {}", x);
    }
//    mutable_variables();


    // scope variables cant be use as global
    fn scope() {
        let prog = "Rust";
        {
            let pre_prog = "Golang";
            println!("Switched Programming lang from {} to {}", prog, pre_prog);
        }
//        println!("It wont work as scope, {}, {}", prog, pre_prog); // !not found in this scope
    }
//    scope();


    fn exercise(){
//        let soldiers= 32;
        const TOTAL_SOLDIERS:i32= 32;
        const RESERVED:i32 = 5;
        let mut soldiers = TOTAL_SOLDIERS;
        let sending= 12;
        println!("{} are in warfield from {}", sending, soldiers);
    //    soldiers = soldiers - 2;
        soldiers -= 2; // better code

        println!("total remaining soldiers are {}", soldiers); // immutable variable
    }
    exercise()

}



fn functions(){
    fn sum(x:i32, y:i32) -> i32{
        x + y
    }
    println!("the sum of 3 and 4 is {}", sum(3,4));

    fn p_energy(mass:f32 , height:f32) -> f32{
        const GRAVITY:f32= 9.8;
    //    let energy = mass * GRAVITY * height;
    //    energy
        mass * GRAVITY * height  // better code
    }
    println!("The potential energy of a 3 kg object at height of 32m is {} joule",
             p_energy(3.0, 32.0));


 


}

