use std::{
    env,
    fs::File,
    io::{self, BufReader},
};

use commands::Command;

mod commands;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: {} --list", args[0]);
        println!("Usage: {} --add", args[0]);
        println!("Usage: {} <alias> [args]", args[0]);
    }

    let commands = fetch_commands_from_file()?;

    if args.len() == 2 && args[1] == "--list" {
        list_commands()?;
        return Ok(());
    }

    if args.len() == 2 && args[1] == "--add" {
        add_command()?;
        return Ok(());
    }

    if args.len() < 2 {
        return Ok(());
    }

    let alias = &args[1];

    let args: Vec<&str> = args.iter().skip(2).map(|s| s.as_str()).collect();

    match find_command_by_alias(&commands, alias) {
        Some(commands) => {
            for cmd in commands {
                if let Err(e) = execute_command(&cmd, &args) {
                    eprintln!("Error executing command: {}\nError: {} ", &cmd, &e);
                }
            }
        }
        None => println!("Command does not exists"),
    }

    Ok(())
}

fn execute_command(cmd: &str, args: &[&str]) -> io::Result<()> {
    let mut modified_command: String = cmd.to_owned();

    for arg in args {
        let _ = modified_command.replace("{}", arg);
    }

    println!("Executing command: {}", modified_command);

    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(&modified_command)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed with exit code {} ", status.code().unwrap_or(-1)),
        ));
    }

    Ok(())
}

fn find_command_by_alias<'a>(commands: &'a [Command], alias: &'a str) -> Option<Vec<&'a str>> {
    for cmd in commands {
        if cmd.alias() == alias {
            return Some(cmd.commands());
        }
    }

    None
}

fn add_command() -> io::Result<()> {
    println!("Enter the alias for your command:");

    let mut alias = String::new();

    io::stdin().read_line(&mut alias)?;

    println!(
        "Enter the commands associated with this alias (one per line) \n empty line to finish"
    );

    let mut commands = Vec::new();

    loop {
        let mut command = String::new();

        io::stdin().read_line(&mut command)?;

        let command = command.trim();

        if command.is_empty() {
            break;
        }

        commands.push(command.to_owned());

        if commands.len() == 0 {
            return Ok(());
        }
    }

    let command = Command::new(alias, commands);

    append_command_to_file(command)?;

    Ok(())
}

fn append_command_to_file(command: Command) -> io::Result<()> {
    let mut commands = fetch_commands_from_file()?;

    commands.push(command);

    let mut file = File::create("commands.json")?;

    serde_json::to_writer_pretty(file, &commands)?;

    Ok(())
}

fn list_commands() -> io::Result<()> {
    let commands = fetch_commands_from_file()?;

    if commands.is_empty() {
        println!("No commands available");
        // return Ok(());
    } else {
        println!("Available commands:\n");

        for cmd in commands {
            println!("\n\tAlias: {}", cmd.alias());

            for comd in cmd.commands() {
                println!("\t {}", comd);
            }
        }
    }

    Ok(())
}

fn fetch_commands_from_file() -> io::Result<Vec<Command>> {
    let file = match File::open("commands.json") {
        Ok(file) => file,
        Err(_) => {
            let _ = File::create("commands.json");
            return Ok(Vec::new());
        }
    };

    let reader = BufReader::new(file);

    let commands = match serde_json::from_reader(reader) {
        Ok(commands) => commands,
        Err(_) => {
            return Ok(Vec::new());
        }
    };

    Ok(commands)
}