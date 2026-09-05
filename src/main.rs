use clap::{Args as _, Command, FromArgMatches as _};
use rip_unburied::args::Commands;
use rip_unburied::env_manager::EnvManager;
use rip_unburied::{args, completions, util};
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let base_cmd = Command::new("rip");
    let cmd = args::Args::augment_args(base_cmd);
    let cli = args::Args::from_arg_matches(&cmd.get_matches()).unwrap();
    let env = EnvManager::default();
    match &cli.command {
        Some(Commands::Completions { shell }) => {
            let result = completions::generate_shell_completions(shell, &mut io::stdout());
            if let Err(e) = result {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        }
        Some(Commands::Graveyard { seance }) => {
            let graveyard = rip_unburied::get_graveyard(None, &env);
            if *seance {
                let cwd = env.current_dir();
                let gravepath = util::join_absolute(
                    graveyard,
                    dunce::canonicalize(cwd).expect("Failed to canonicalize the current directory"),
                );
                println!("{}", gravepath.display());
            } else {
                println!("{}", graveyard.display());
            }
        }
        None => {
            let mut stream = io::stdout();
            let mode = util::ProductionMode;

            ////////////////////////////////////////////////////////////
            // Main code ///////////////////////////////////////////////
            let result = rip_unburied::run(&cli, mode, &mut stream, &env);
            ////////////////////////////////////////////////////////////

            if let Err(ref e) = result {
                println!("Exception: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
