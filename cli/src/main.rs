use clap::{Command, Arg, error::ErrorKind, ArgMatches};
use calco::{compiler::interpreter::Interpreter as Engine, ast::Compile};

fn create_cli_parser() -> Command {
    Command::new("calco")
        .display_name("Calco")
        .bin_name("calco")
        .author("Sedorriku")
        .version("0.1.0")
        .about("Use blazingly fast algorithms")
        .subcommand(
            Command::new("run")
            .about("Run a source from a file or a string")
            .arg(Arg::new("source").short('x').long("execute").required(true).allow_negative_numbers(true).allow_hyphen_values(true))
            .arg(Arg::new("variable").alias("var").long("variable").required(false))
        ).subcommand_required(true)
}

fn main(){
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut cli = create_cli_parser();
    let args = cli.clone().try_get_matches();

    match args {
        Ok(parsed_args) => {
            cli_executor(parsed_args, &mut cli);
        },
        Err(err) => {
            match err.kind() {
                ErrorKind::DisplayHelp => { let _ = cli.print_help(); },
                ErrorKind::DisplayVersion => { println!("{}", cli.get_version().unwrap_or("0.0.0")); },
                _ => err.exit()
            };
        }
    };

}

fn cli_executor(args: ArgMatches, cli: &mut Command){
    match args.subcommand() {
        Some(("run", args)) => {
            if let Some(source) = args.get_one::<String>("source") {
                if source.starts_with("-") {
                    // add 0 to block EOI error
                    run(format!("0{}", source))
                } else {
                    run(source.to_string())
                }
            }
        },
        Some((unvalid, _)) => {
            cli.error(ErrorKind::InvalidSubcommand, format!("Cannot recognize {unvalid} as an available subcommand"));
        }
        None => {
            cli.error(ErrorKind::MissingSubcommand, "No subcommand was provided");
        }
    }
}

fn run(source: String){
    let output = Engine::from_source(source.as_str());

    match output {
        Ok(Ok(res)) => messages::success(res),
        Ok(Err(execution_error)) => messages::print_error(execution_error),
        Err(err) => messages::error(err)
    }
}

mod messages {
    use std::fmt::Display;

    use calco::compiler::interpreter::EvalError;


    pub (in super) fn success(result: impl Display){
        println!("{result}")
    }

    pub (in super) fn print_error(error: EvalError){
        match error {
            EvalError::MultiplicationSyntax => self::error("Cannot multiplicate a number without another number"),
            EvalError::DivisionByZero => self::error("Cannot divide a number by zero"),
            EvalError::DivisionSyntax => self::error("Cannot divide a number without another number"),
        }
    }

    pub (in super) fn error(error: impl Display){
        println!("\x1b[31mERROR!\x1b[0m {error}")
    }
}