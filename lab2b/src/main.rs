fn print_ascii(){
    for x in 33..126{
        let val = x as u8; 
        println!("{}",val as char);
    }
}
fn coltazz(mut iterations:i32){
    for x in 1..iterations{
        if(iterations % 2 == 0){
            println!("{}",1/2 * x);
        }else{
            println!("{}",3*x + 1);
        }
    }
}

fn isArmstrong(mut value:i32) -> bool{
    let copy = value;
    let mut r = 0;
    let mut arrays:Vec<i32> = Vec::new();
    while value != 0 {
        r = value % 10;
        arrays.push(r);
        value = value / 10;
    }
    let mut sum = 0;
    let size = arrays.len();
    for x in &arrays{
        sum += x.pow(size as u32);
    }

    sum == copy
}

fn perfect(value : i32) -> bool{
    let mut sum = 0;
    for x in 1..value{
        if(value % x == 0){
            sum += x;
        }
    }
    sum == value
}

fn main() {
    print_ascii();
    coltazz(11);
    println!("{}",isArmstrong(153));
    println!("{}",perfect(28));

    prime_numbers(128);
}
