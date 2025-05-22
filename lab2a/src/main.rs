fn factorial_while(value: u32 ) -> u32{
    let mut result = 1;
    let mut count = 1;
    while count < value + 1 {
        result *= count;
        count+=1;
    }
    return result;
}
fn factorial_loop(value: u32 ) -> u32{
    let mut result = 1;
    let mut count = 1;
    
    loop{
        if count > value{
            break;
        } 
        result *= count;
        count += 1;
    }
    return result;
}

fn main() {
    println!("{}",factorial_while(3));
    println!("{}",factorial_loop(3));
}
