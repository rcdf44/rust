fn main() {
    let a = "hola mundo";
    println!("{}", a);

    let mut b = 19;

    println!("el valor de b es: {}", b);

    b = 18;

    println!("ahora el valor de b es: {}", b);

    let c = {
        let a = 98 + b;


        fn t(letra: &str){
            println!("{}", letra);
        }

        t("juan");

        a

    };


    println!("{}", c);

    let n: [u32; 5] = r();

    if b > 20 {
        println!("B es mayor que veinte");
    }else{
        println!("B no es mayor que veinte");
    }
 
    let d = retornar(b);

    let data: u32 = if b > 20 {10} else {b};

    println!("{}", d);

}

fn retornar(n: i32) -> i32 {
    let a: i32 = 15 - n;

    a 
} 

fn r () -> [u32; 5] {
    let c: [u32; 5] = [1, 2, 3, 4, 5];

    c
}