use std::io;

fn main() {

    println!("Enter a vector");
    let mut vector = String::new();
    loop{
    io::stdin().read_line(&mut vector).expect("Error");
    let vector: u32 = match vector.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    
    if vector % 2 == 0 {
        let result = vector / 2;
        println!("{}", result);
        break;
    }
    else {
        let exp = 2;
        let result = vector.pow(exp);
        println!("{}", result);
        break;
    }
    }

}