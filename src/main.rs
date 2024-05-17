use std::{
    env,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

use commands::Command;

mod commands;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: prkcst --list [--global]");
        println!("Usage: prkcst --add [--global]");
        println!("Usage: prkcst <alias> [args] [--global]");
        return Ok(());
    }

    let global_flag = args.iter().any(|arg| arg == "--global" || arg == "-g");

    if args.len() >= 2 && args[1] == "--list" {
        list_commands(global_flag)?;
        return Ok(());
    }

    if args.len() >= 2 && args[1] == "--add" {
        add_command(global_flag)?;
        return Ok(());
    }

    if args.len() < 2 {
        return Ok(());
    }

    let commands = fetch_commands_from_file(global_flag)?;

    let alias = &args[1];

    let mut args: Vec<&str> = args.iter().skip(2).map(|s| s.as_str()).collect();

    if global_flag {
        args.remove(args.len() - 1);
    }

    match find_command_by_alias(&commands, alias) {
        Some(commands) => {
            let args_required = commands
                .iter()
                .map(|f| f.matches("{}").count())
                .sum::<usize>();
            if args_required != args.len() && args_required > 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Invalid arguments count, please provide exactly {} arguments",
                        args_required
                    ),
                ));
            }
            let mut args_idx = 0;

            for cmd in commands {
                let args_required = cmd.matches("{}").count();

                let args_for_command = match (args.is_empty(), args_required) {
                    (true, _) => Vec::new(),
                    (false, 0) => Vec::new(),
                    (false, 1) => vec![args[args_idx]],
                    _ => args[args_idx..(args_idx + args_required)].to_vec(),
                };

                args_idx += args_required;

                if let Err(e) = execute_command(&cmd, &args_for_command) {
                    eprintln!("Error executing command: {}\nError: {} ", &cmd, &e);
                }
            }
        }
        None => println!("Command does not exist"),
    }

    Ok(())
}

fn execute_command(cmd: &str, args: &[&str]) -> io::Result<()> {
    let mut modified_command: String = cmd.to_owned();
    if modified_command.matches("{}").count() > 0
        && modified_command.matches("{}").count() != args.len()
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Invalid arguments count, please provide exactly {} arguments",
                args.len()
            ),
        ));
    }

    for arg in args {
        modified_command = modified_command.replacen("{}", arg, 1);
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

fn add_command(global: bool) -> io::Result<()> {
    println!("Enter the alias for your command:");

    let mut alias = String::new();

    io::stdin().read_line(&mut alias)?;

    let alias = alias.trim().to_owned();

    println!(
        "Enter the commands associated with this alias (one command per line) \nEnter an empty line to finish"
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
    }

    let command = Command::new(alias.clone(), commands.clone());

    if global {
        append_command_to_file(command, true)?;
    } else {
        append_command_to_file(command, false)?;
    }

    Ok(())
}

fn list_commands(global: bool) -> io::Result<()> {
    let commands = fetch_commands_from_file(global)?;

    if commands.is_empty() {
        println!("No commands available");
    } else {
        println!("Available commands:\n");

        for cmd in commands {
            println!("Alias: {}", cmd.alias());
            println!("Commands:");
            for comd in cmd.commands() {
                println!("{}", comd);
            }
            println!();
        }
    }

    Ok(())
}

fn fetch_commands_from_file(global: bool) -> io::Result<Vec<Command>> {
    let local_path = "./commands.json"; // Local path relative to the current directory
    let global_path = get_global_commands_path(); // Get the global commands path

    let path: PathBuf = if global {
        global_path.to_owned()
    } else {
        local_path.to_owned().into()
    };

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            // if global {
            //     File::create(&global_path)?; // Create global file if it doesn't exist
            // } else {
            //     File::create(&local_path)?; // Create local file if it doesn't exist
            // }
            return Ok(Vec::new());
        }
    };

    let reader = BufReader::new(file);
    let commands = serde_json::from_reader(reader)?;

    Ok(commands)
}

fn append_command_to_file(command: Command, global: bool) -> io::Result<()> {
    let local_path = "./commands.json"; // Local path relative to the current directory
    let global_path = get_global_commands_path(); // Get the global commands path

    let mut commands = fetch_commands_from_file(global)?;

    commands.push(command);

    let path: PathBuf = if global {
        global_path.to_owned()
    } else {
        local_path.to_owned().into()
    };

    let file = File::create(&path)?;
    serde_json::to_writer_pretty(file, &commands)?;

    Ok(())
}

fn get_global_commands_path() -> PathBuf {
    // Example: Get the directory where the executable is stored
    let mut path = env::current_exe().expect("Failed to get current executable path");
    path.pop(); // Remove executable name, keep directory
    path.push("commands.json"); // Append commands.json to the directory
    path
}
