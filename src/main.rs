use std::io::{self, BufWriter, Write};
use std::fs::{read, File, OpenOptions};
use std::path::{self, Path};
use serde::{Deserialize, Serialize};
use std::io::BufReader;

#[derive(Deserialize, Serialize)]
struct ApiInfo{
    api_key : String,
    model : String,
}
    
fn main()-> Result<(), Box<dyn std::error::Error>>{
        
        
    let path_to_file = std::path::Path::new("config.json");
    match File::open(path_to_file){
        Ok(file) => file,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound  => {File::create(path_to_file)?},
        Err(e) => {println!("Error: {}", e); return Err(Box::new(e));}
    };

    // ? reading the json file
    let extracted_info: ApiInfo = match read_json(path_to_file){
        Ok(values) => values,
        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {info()},
        Err(e) => return Err(Box::new(e))
    };

    let _ = write_json(path_to_file, &extracted_info);
    println!("\nApi: {}\nModel: {}", extracted_info.api_key, extracted_info.model);

    Ok(())
}

fn read_json(file_path : &Path) -> Result<ApiInfo, io::Error>{

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let extractor = serde_json::from_reader(reader)?;
    Ok(extractor)
}

fn write_json(file_path : &Path, data: &ApiInfo) -> Result<(), io::Error>{
    let file = OpenOptions::new().write(true).truncate(true).open(file_path)?;

    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer(&mut writer, &data)?;
    Ok(())
}


fn info() -> ApiInfo {
    // ? Getting the api key
    let mut api_key = String::new();
    print!("Enter your Api Key: ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin().read_line(&mut api_key)
        .expect("Failed to read Api Key");

    let api_key: String = api_key.trim().parse()
        .expect("Failed to parse the api key");


// ? Getting the model choice 
    let mut model_choice = String::new();
    loop{
        println!("-------------------------------------------------");
        println!("  1 : LLaMA3 8b");
        println!("  2 : Mixtral 8x7b");
        println!("  3 : LLAMA3 70B");
        println!("  4 : Gemma 7b");
        print!("  Select your model: " );
        io::stdout().flush().expect("Failed to flush the print buffer");

        model_choice.clear();
        io::stdin().read_line(&mut model_choice)
            .expect("Can't read the model input");

        match model_choice.trim().parse::<i32>(){
            Ok(1) => {model_choice = "LLaMA3 8b".to_string(); break},
            Ok(2) => {model_choice = "Mixtral 8x7b".to_string(); break},
            Ok(3) => {model_choice = "LLaMA3 70b".to_string(); break},
            Ok(4) => {model_choice = "Gemma 7b".to_string(); break},
            Ok(_) => {println!("    Enter an option between 1-4"); continue},
            Err(_) => {println!("   Enter a valid option"); continue},
        };
        
    }

    // ? Adding the final choice to the struct
    let final_choice = ApiInfo{
        api_key : api_key,
        model : model_choice
    };

    return final_choice
}