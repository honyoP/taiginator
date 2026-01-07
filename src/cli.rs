use clap::{Arg, ArgAction, Command, Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.1")]
#[command(about = "A taigination of task manager.")]
pub struct Args {
    #[command(subcommand)]
    action: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        query: Vec<String>,
    },
    List,
}

pub fn cli() -> Command {
    Command::new("taiga")
        .about(concat!(
            ">>=====================================<<
|| /__  ___/                           ||
||   / /   ___     ( )  ___      ___   ||
||  / /  //   ) ) / / //   ) ) //   ) )||
|| / /  //   / / / / ((___/ / //   / / ||
||/ /  ((___( ( / /   //__   ((___( (  ||
>>=====================================<<\n",
            "~A task organizer from a mentally deficit monkey~"
        ))
        .version(env!("CARGO_PKG_VERSION"))
        .long_version(concat!(
            "v",
            env!("CARGO_PKG_VERSION"),
            "\nCodeName: ",
            env!("CODENAME")
        ))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds a task")
                .subcommand_precedence_over_arg(true)
                .arg(
                    Arg::new("TITLE")
                        .help("Title for task")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .required(true),
                )
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("when").about("Schedules a task").arg(
                        Arg::new("SCHEDULED")
                            .help("Schedules a task for certain date")
                            .action(ArgAction::Set)
                            .num_args(1..)
                            .required(false),
                    ),
                ),
        )
        .subcommand(
            Command::new("list").about("Lists tasks").arg(
                Arg::new("STATE")
                    .help("Filter task by state")
                    .action(ArgAction::Set)
                    .num_args(1..)
                    .required(false),
            ),
        )
        .subcommand(
            Command::new("check").about("Checks task completed.").arg(
                Arg::new("ID")
                    .help("Task ID to be checked complete.")
                    .action(ArgAction::Set)
                    .num_args(1)
                    .required(true)
                    .value_parser(clap::value_parser!(u32)),
            ),
        )
        .subcommand(
            Command::new("remove").about("Removes a task").arg(
                Arg::new("ID")
                    .help("Task ID to be removed")
                    .action(ArgAction::Set)
                    .num_args(1..)
                    .required(true)
                    .value_parser(clap::value_parser!(u32)),
            ),
        )
        .subcommand(
            Command::new("pomo")
                .about("Pomodoro manager.")
                .subcommand_precedence_over_arg(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("start")
                        .about("Starts new pomodoro session.")
                        .arg(
                            Arg::new("FOCUS")
                                .help("How long should focus session last.")
                                .action(ArgAction::Set)
                                .num_args(1)
                                .required(true),
                        )
                        .arg(
                            Arg::new("BREAK")
                                .help("How long should break session last.")
                                .action(ArgAction::Set)
                                .num_args(1)
                                .required(true),
                        )
                        .arg(
                            Arg::new("CYCLES")
                                .help("How many cycles of focus times to repeat.")
                                .action(ArgAction::Set)
                                .num_args(1)
                                .required(true),
                        ),
                )
                .subcommand(Command::new("status").about("Shows status of running session."))
                .subcommand(Command::new("stop").about("Stops running pomodoro session."))
                .subcommand(Command::new("pause").about("Pauses running pomodoro session."))
                .subcommand(Command::new("resume").about("Resumes paused pomodoro session."))
                .subcommand(Command::new("kill").about("Kills daemon.")),
        )
        .subcommand(Command::new("daemon").hide(true))
}
