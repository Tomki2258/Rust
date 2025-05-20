fn leap(year : u32){
    if year % 4 == 0{
        println!("Rok jest przestępny");
    }
    else{
        println!("Rok NIE jest przestępny");
    }
}

fn days_amount(month:u32,year:u32) -> u32{
    if month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12 {
        return 31;
    }
    else{
        if(month == 2){
            if year % 4 == 0 {
            return 28;
        }else{
            return 29;
        }
        return 30;
    }
    return 0;  
    }
}

fn to_fahrenheit(celcius:f32) -> f32{
    let result = 32.0 + 9.0/5.0 * celcius;
    return result;
}

fn to_celcius(fahrenheit:f32) -> f32{
    fahrenheit * 5.0 / 9.0 - 32.0
}

fn calculate_time_difference(h1:i32,m1:i32,s1:i32,h2:i32,m2:i32,s2:i32){
    let hDifference = h1 - h2;
    let mDifference = m1 - m2;
    let sDifference = s1 - s2;


    println!("Time difference H:{} M:{} S:{}",
        hDifference.abs()
        ,mDifference.abs()
        ,sDifference.abs());
}

fn factorial(value: u32 ) -> u32{
    let mut result = 1;
    for x in 1..value + 1{
        result *= x;
    }
    return result;
}

fn print_digits(mut value:u32){
    let mut r = 0;
    while value != 0 {
        r = value % 10;
        println!("{}",r);
        value = value / 10;
    }
}

fn sum_digits(mut value:u32) -> u32{
    let mut r = 0;
    let mut sum = 0;
    while value != 0 {
        r = value % 10;
        sum += r;
        value = value / 10;
    }
    sum
}


fn main() {
    leap(2024);

    println!("{}",days_amount(2, 2025));

    println!("{}",to_fahrenheit(15.0));

    println!("{}",to_celcius(15.0));

    calculate_time_difference(15,25, 10, 20, 15, 5);

    println!("Factorial:{}",factorial(3));

    print_digits(54367);

    println!("{}",sum_digits(1234));
}