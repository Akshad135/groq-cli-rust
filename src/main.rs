use std::io::{self, BufReader, BufWriter, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use reqwest::Client;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("HTTP request error")]
    HttpRequest(#[from] reqwest::Error),
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Deserialize, Serialize)]
struct ApiInfo {
    api_key: String,
    model: String,
}

#[derive(Deserialize)]
struct ApiResponse { 
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

#[derive(Deserialize, Serialize)]
struct JsonRequest {
    messages: Vec<MessagePart>,
    model: String,
}

#[derive(Deserialize, Serialize)]
struct MessagePart {
    role: String,
    content: String,
}


fn main() -> Result<(), MyError> {
    let args : Vec<String> = std::env::args().collect();
    let path_to_file = Path::new("config.json");

    if args.len() == 2 {
        match args[1].as_str() {
            "-s" | "--setup" => {
                info(&path_to_file)?;
            }
            "-h" | "--help" => {
                help();
                return Ok(());
            }
            arg => {
                println!("Unknown argument: \"{}\"", arg);
                help();
                return Ok(());
            }
        }
    } else if args.len() == 1 {
        entry()?;
    } else {
        help();
    }

    Ok(())
}

fn help(){
    let cmd = "mycli";
    println!("{} [--s | --setup] : Initialize the setup", cmd);
    println!("{} [--h | --help] : Shows avaiable cmds", cmd);
}

fn entry() -> Result<(), MyError> {
    let path_to_file = Path::new("config.json");

    // Check if the file exists
    if !path_to_file.exists() {
        // File does not exist, create and get info
        File::create(&path_to_file)?;
        info(&path_to_file)?;
    } else {
        // File exists, check if it contains valid info
        let metadata = fs::metadata(&path_to_file)?;
        let file_size = metadata.len();
        if file_size == 0 {
            info(&path_to_file)?
        } else {
            match read_json(&path_to_file) {
                Ok(info) => info,
                Err(_) => info(&path_to_file)?,
            }
        };
    }
    call_api(&path_to_file)?;
    Ok(())
}

fn read_json(file_path: &Path) -> Result<ApiInfo, MyError> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let extractor = serde_json::from_reader(reader)?;
    Ok(extractor)
}

fn write_json(file_path: &Path, data: &ApiInfo) -> Result<(), MyError> {
    let file = OpenOptions::new().write(true).truncate(true).open(file_path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &data)?;
    Ok(())
}

fn info(path_to_file: &Path) -> Result<ApiInfo, MyError> {
    // Getting the api key
    println!("Initializing setup");
    let mut api_key = String::new();
    print!("Enter your Api Key: ");
    io::stdout().flush()?;

    io::stdin().read_line(&mut api_key)?;
    let api_key: String = format!("Bearer {}", api_key.trim());

    // Getting the model choice
    let model_choice = loop {
        println!("-------------------------------------------------");
        println!("  1 : LLaMA3 8b");
        println!("  2 : Mixtral 8x7b");
        println!("  3 : LLAMA3 70B");
        println!("  4 : Gemma 7b");
        print!("  Select your model: ");
        io::stdout().flush()?;

        let mut model_choice = String::new();
        io::stdin().read_line(&mut model_choice)?;
        let model_choice = model_choice.trim();

        match model_choice {
            "1" => break "llama3-8b-8192".to_owned(),
            "2" => break "mixtral-8x7b-32768".to_owned(),
            "3" => break "llama3-70b-8192".to_owned(),
            "4" => break "gemma-7b-it".to_owned(),
            _ => println!("Enter an option between 1-4"),
        };
    };

    // Adding the final choice to the struct
    let final_choice = ApiInfo {
        api_key,
        model: model_choice,
    };
    write_json(path_to_file, &final_choice)?;
    Ok(final_choice)
}

#[tokio::main]
async fn call_api(file_path : &Path) -> Result<(), MyError>{
    let extracted_info = read_json(file_path)?;
    let my_token = extracted_info.api_key;

    print!("Enter your query: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut data_content = String::new();
    io::stdin().read_line(&mut data_content)
        .expect("Failed to read query");

    let data_content = data_content.trim().parse()
        .expect("Failed to parse as string");

    let message_data = MessagePart {
        role: "user".to_owned(),
        content: data_content,
    };

    let json_data = JsonRequest {
        messages: vec![message_data],
        model: extracted_info.model,
    };

    let json_data = serde_json::to_string(&json_data)?;

    let client = Client::new();
    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", my_token)
        .header("Content-Type", "application/json")
        .body(json_data)
        .send()
        .await?;

    let body_text = response.text().await?;
    let api_response: ApiResponse = serde_json::from_str(&body_text)?;

    for choice in api_response.choices {
        println!("-----------------------------------------------------------------------------------");
        println!("{}", choice.message.content);
    }

    Ok(())
}