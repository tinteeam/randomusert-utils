use std::process::Command;
use std::io::{self, Write};

fn main() {
    cfg!(target_os = "windows");
    println!(">");
    loop {
        let mut input = String::new();

        // Read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim(); // Remove extra whitespace

        if trimmed_input.is_empty() {
            eprintln!("Error: No command entered.");
            return;
        }

        // Execute the command via cmd.exe
        match Command::new("cmd")
            .arg("/C") // Pass the command to the shell
            .arg(trimmed_input)
            .spawn()
        {
          Ok(mut child) => {
             child.wait().expect("Failed to wait on child process");
            }
            Err(error) => {
                eprintln!("Error: Failed to execute '{}': {}", trimmed_input, error);
            }
        }
    }
    cfg!(target_os = "linux");
    println!(">");
    loop {
        


        let mut input = String::new();

        // Read user input
        io::stdin()
         .read_line(&mut input)
           .expect("Failed to read line");

        let trimmed_input = input.trim(); // Remove extra whitespace

        if trimmed_input.is_empty() {
            eprintln!("Error: No command entered.");
            return;
        }

        // Execute the command via shell
        match Command::new("sh")
            .arg("-c") // Pass the command to the shell
            .arg(trimmed_input)
            .spawn()
        {
            Ok(mut child) => {
                child.wait().expect("Failed to wait on child process");
            }
            Err(error) => {
                eprintln!("Error: Failed to execute '{}': {}", trimmed_input, error);
            }
            }
        }

}    

