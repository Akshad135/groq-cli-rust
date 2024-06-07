use std::io::{self, Write};


fn main(){

// ? Getting the api key
    let mut api_key = String::new();
    print!("Enter your Api Key: ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin().read_line(&mut api_key)
        .expect("Failed to read Api Key");

    let api_key: String = api_key.trim().parse()
        .expect("Failed to parse the api key");

    println!("{}", api_key);

// ? Getting the model choice 
    let models = vec![
        "LLaMA3 8b",
        "LLaMA3 70b",
        "Mixtral 8x7b",
        "Gemma 7b"
    ];
    let mut model_choice_input = String::new();
    let mut model_choice = String::new();
    loop{
        println!("-------------------------------------------------");
        println!("  1 : LLaMA3 8b");
        println!("  2 : Mixtral 8x7b");
        println!("  3 : LLAMA3 70B");
        println!("  4 : Gemma 7b");
        print!("  Select your model: " );
        io::stdout().flush().expect("Failed to flush the print buffer");

    
        io::stdin().read_line(&mut model_choice_input)
            .expect("Can't read the model input");

        let model_choice_input = match model_choice_input.trim().parse::<i32>(){
            Ok(1) => {model_choice = "LLaMA3 8b".to_string(); break},
            Ok(2) => {model_choice = "Mixtral 8x7b".to_string(); break},
            Ok(3) => {model_choice = "LLaMA3 70b".to_string(); break},
            Ok(4) => {model_choice = "Gemma 7b".to_string(); break},
            Ok(_) => {println!("    Enter an option between 1-4"); continue},
            Err(_) => {println!("Invalid int"); continue},
        };
        
    }
    println!("{}", model_choice);
}