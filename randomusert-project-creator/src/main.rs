use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use std::process;

fn main() {
    // Command-line arguments: language and project name
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <language> <project_name>", args[0]);
        process::exit(1);
    }

    let language = &args[1].to_lowercase();
    let project_name = &args[2];

    match create_project(language, project_name) {
        Ok(_) => println!("Project '{}' created successfully!", project_name),
        Err(err) => eprintln!("Error creating project: {}", err),
    }
}

fn create_project(language: &str, project_name: &str) -> std::io::Result<()> {
    // Create project directory
    let project_dir = Path::new(project_name);
    create_dir_all(project_dir)?;

    // Generate boilerplate based on language
    match language.as_ref() {
        "c" => create_c_project(project_dir)?,
        "cpp" | "c++" => create_cpp_project(project_dir)?,
        "python" => create_python_project(project_dir)?,
        "rust" => create_rust_project(project_dir)?,
        "javascript" | "js" => create_javascript_project(project_dir)?,
        "csharp" | "c#" => create_csharp_project(project_dir)?,
        _ => {
            eprintln!("Unsupported language: {}", language);
            process::exit(1);
        }
    }

    Ok(())
}

fn create_c_project(project_dir: &Path) -> std::io::Result<()> {
    let src_path = project_dir.join("src");
    create_dir_all(&src_path)?;

    let main_c = src_path.join("main.c");
    let mut file = File::create(main_c)?;
    file.write_all(
        b"#include <stdio.h>\n\nint main() {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}\n",
    )?;

    println!("C project created.");
    Ok(())
}

fn create_cpp_project(project_dir: &Path) -> std::io::Result<()> {
    let src_path = project_dir.join("src");
    create_dir_all(&src_path)?;

    let main_cpp = src_path.join("main.cpp");
    let mut file = File::create(main_cpp)?;
    file.write_all(
        b"#include <iostream>\n\nint main() {\n    std::cout << \"Hello, World!\" << std::endl;\n    return 0;\n}\n",
    )?;

    println!("C++ project created.");
    Ok(())
}

fn create_python_project(project_dir: &Path) -> std::io::Result<()> {
    let main_py = project_dir.join("main.py");
    let mut file = File::create(main_py)?;
    file.write_all(b"print(\"Hello, World!\")\n")?;

    println!("Python project created.");
    Ok(())
}

fn create_rust_project(project_dir: &Path) -> std::io::Result<()> {
    let src_path = project_dir.join("src");
    create_dir_all(&src_path)?;

    let main_rs = src_path.join("main.rs");
    let mut file = File::create(main_rs)?;
    file.write_all(b"fn main() {\n    println!(\"Hello, World!\");\n}\n")?;

    println!("Rust project created.");
    Ok(())
}

fn create_javascript_project(project_dir: &Path) -> std::io::Result<()> {
    let main_js = project_dir.join("main.js");
    let mut file = File::create(main_js)?;
    file.write_all(b"console.log(\"Hello, World!\");\n")?;

    println!("JavaScript project created.");
    Ok(())
}

fn create_csharp_project(project_dir: &Path) -> std::io::Result<()> {
    let src_path = project_dir.join("src");
    create_dir_all(&src_path)?;

    let main_cs = src_path.join("Program.cs");
    let mut file = File::create(main_cs)?;
    file.write_all(
        b"using System;\n\nclass Program {\n    static void Main() {\n        Console.WriteLine(\"Hello, World!\");\n    }\n}\n",
    )?;

    println!("C# project created.");
    Ok(())
}
