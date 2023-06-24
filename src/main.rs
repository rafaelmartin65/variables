fn main() {

    //Integer
    let entero: i8 = 23;
    let entero1: u8 = 40;
    let entero2: i8 = -20;

    //Integer decimal
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;

    //Floating point
    let float1 = 5.0;  //f64
    let float32: f32 = 12.432;  //f32

    //Boolean
    let verdadero = true;
    let falso: bool = false;

    //Character
    let caracter = 'a';

    // Tuplas
    let tupla = ('h', 23, -45, 0.222);
    let tupla2:(char, i64, i32, f64) = ('h', 23, -45, 0.222);

    // Array
    let arreglo = [1, 2, 3, 4, 5];
    

    let (x,y,z,w) = tupla;

    println!("el primer valor de la tupla es: {}", tupla.1);


    let x = "____";
    
    println!("variable value is equal: {}", x);

    let x = x.len();

    assert_eq!(4,x);
    println!("...OK!  2nd value variable length is equal: {}", x);


    let mut x = 5;
    println!("{x}");
    let x = 10;
    println!("{x}");

    const THREE_HOURS_IN_SECONDS:u32 = 3 * 60 *60;

    println!("{THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("the value of 'y' in the inner scope is : {y}");
    }

    println!("the value of 'y'  is: {y}");

}
