
# Command Shortcut Tool

Command Shortcut Tool is a command-line interface (CLI) tool written in Rust that allows users to define and execute custom commands easily.

## Features

- **Add Commands**: Dynamically add new commands with custom aliases and associated commands.
- **List Commands**: View a list of all available commands along with their aliases and associated commands.
- **Execute Commands**: Execute predefined commands by specifying their aliases.

## Installation

    ```bash
    cargo install prkcst
    ```


## Usage

### Adding a Command

To add a new command, run the CLI tool with the `--add` option:

```bash
./prkcst --add

Enter the alias for your command:
hello

Enter the commands associated with this alias (one command per line)\n Enter an empty line to finish

echo hello world

```
Similarly, you can also have arguments for commands as: 
```bash
./prkcst --add

Enter the alias for your command:
hello_user

Enter the commands associated with this alias (one command per line)\n Enter an empty line to finish

echo hello {}

```

Follow the prompts to enter the alias and associated commands for the new command. Use "{}" as an argument placeholder.

### Listing Commands

To list all available commands, run the CLI tool with the `--list` option:

```bash
./prkcst --list
```

### Executing a Command

To execute a predefined command, specify its alias along with any arguments:

```bash
./prkcst <alias> [args]
```

Replace `<alias>` with the alias of the command and `[args]` with any arguments required by the command.

## License

This project is licensed under the [MIT License](LICENSE).

