fn main() {
    let a = "hola mundo";
    println!("{}", a);

    let mut b = 19;

    println!("el valor de b es: {}", b);

    b = 18;

    println!("ahora el valor de b es: {}", b);

    let c = {
        let a = 98 + b;

        println!("{}",a);

        fn t(letra: &str){
            println!("{}", letra);
        }

        t("juan");

        a

    };

    let d = retornar(b);

    println!("{}", d);

}

fn retornar(n: i32) -> i32 {
    let a: i32 = 15 - n;

    a 
}