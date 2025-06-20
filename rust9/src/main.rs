use std::fs::File;

fn main() {
    println!("Hello, world!");

    let open_file=File::open("check.txt");

    let opened_file= match open_file{
        Ok(file)=>file,
        Err(_)=>panic!("file not found")
    };
    
}

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             _ => {
//                 panic!("Problem opening the file: {error:?}");
//             }
//         },
//     };
// }