mod codes;
mod crack;
use std::process::exit;

use codes::CodesFile;
use crack::Crack;

fn main() {
    if !CodesFile::file_exists() {
        CodesFile::create_file();
        println!("Please add the codes to the created file and run the program again.");
        println!("The file is located at ./file.json");
        exit(0);
    }
    let codes: Vec<Crack> = Crack::create_codes();
    let mut possible_codes: Vec<Vec<i8>> = Crack::create_possible_codes(&codes);
    Crack::remove_everything_with_0_0(&codes, &mut possible_codes);
    let mut counter = 0;
    show_start(&codes);

    loop {
        counter = counter % codes.len();
        let current_code = &codes[counter];
        // if current_code.correct_place == 0 && current_code.wrong_place == 0 {
        //     counter += 1;
        //     continue;
        // }

        current_code.keep_correct_place(&mut possible_codes);
        current_code.remove_wrong_wrong_place(&mut possible_codes);

        counter += 1;
        if counter == codes.len() {
            break;
        }
    }

    match possible_codes.len() {
        0 => println!("No possible codes"),
        1 => println!("The code is {:?}", possible_codes[0]),
        _ => {
            println!("There are multiple possible codes");
            for code in possible_codes {
                println!("Possible: {:?}", code);
            }
        }
    }
}

fn show_start(codes: &Vec<Crack>) {
    println!("This program will try to crack the code.");
    println!("(Possible it will not work, but it's a start)\n");
    println!("The codes are:");
    for code in codes {
        println!("{:?}", code);
    }
    println!();
}
