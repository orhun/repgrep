mod cli;
mod encoding;
mod model;
mod replace;
mod rg;
mod ui;
mod util;

use std::collections::VecDeque;
use std::io;
use std::process;

use cli::Args;
use rg::de::RgMessage;
use rg::exec::run_ripgrep;
use rg::read::read_messages;
use ui::tui::Tui;

fn print_and_exit(message: impl AsRef<str>, print_help: bool) {
    if print_help {
        cli::print_help();
    }

    eprintln!("{}", message.as_ref());
    process::exit(1);
}

fn start_app(args: &Args, rg_messages: VecDeque<RgMessage>) {
    let rg_cmdline: String = args
        .rg_args()
        .map(|s| s.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(" ");

    let result = Tui::new(rg_cmdline, rg_messages).start();

    // Restore terminal.
    if let Err(err) = Tui::restore_terminal() {
        eprintln!(
            "Failed to restore terminal state, consider running the `reset` command. Error: {}",
            err
        );
    }

    // Handle application result.
    match result {
        Ok(Some(mut replacement_criteria)) => {
            // If we detected an encoding passed to `rg`, then use that.
            if let Some(ref encoding) = args.encoding {
                replacement_criteria.set_encoding(encoding);
            }

            match replace::perform_replacements(replacement_criteria) {
                Ok(_) => {}
                Err(err) => print_and_exit(
                    format!("An error occurred during replacement: {}", err),
                    false,
                ),
            }
        }
        Ok(None) => eprintln!("Cancelled"),
        Err(err) => print_and_exit(format!("An app error occurred: {}", err), false),
    }
}

fn main() {
    match cli::parse_arguments() {
        // Normal invocation with arguments.
        Ok(args) => match run_ripgrep(args.rg_args()) {
            Ok(rg_messages) => start_app(&args, rg_messages),
            Err(e) => print_and_exit(format!("{}", e), false),
        },
        // Either an error, or running without any arguments (reads from STDIN).
        Err(e) => match e.downcast_ref::<cli::NoArgumentsError>() {
            Some(cli::NoArgumentsError) => {
                eprintln!("No arguments were provided, reading from STDIN...");
                let args = &Args::default();
                let stdin = io::stdin();
                match read_messages(stdin.lock()) {
                    Ok(rg_messages) => start_app(args, rg_messages),
                    Err(e) => print_and_exit(
                        format!("\nFailed to parse input from STDIN, error: {}", e),
                        true,
                    ),
                }
            }
            None => {
                print_and_exit(format!("\nFailed to parse arguments, error: {}", e), true);
            }
        },
    };
}
