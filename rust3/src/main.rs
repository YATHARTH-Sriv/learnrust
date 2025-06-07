use std::io;
fn main(){
    println!("Welcome To Temp Converter");
    let mut temperature_in_celsius=String::new();

    println!("Enter temp in celsius");

    io::stdin()
        .read_line(&mut temperature_in_celsius)
        .expect("Failed to read line");

    println!("temp entered : {}",temperature_in_celsius);

    let temperature_in_celsius:i32= match temperature_in_celsius.trim().parse(){
        Ok(temp)=>temp,
        Err(_)=>{
            println!("Enter valid temp");
            0
        }
    };

    let temp_in_fahrenheit=convert_to_fahrenheit(temperature_in_celsius);
    println!("temperature in Fahrenheit is {}",temp_in_fahrenheit)

}

fn convert_to_fahrenheit(temp:i32)->i32{
    (temp*9/5) +32
}