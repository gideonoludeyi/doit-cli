use super::task::{complete, delete, get_all, save, undo, Task};
use super::util;
use clap::{arg, App, AppSettings, ArgMatches};
use std::error::Error;

pub fn build(name: &str) -> App<'static> {
    App::new(name)
        .about("task command module")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .subcommand(
            App::new("add")
                .about("adds a task")
                .arg(arg!(<NAME> "The name of the task"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(
            App::new("del")
                .about("deletes a task")
                .arg(arg!(<ID> "The id of the task"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(
            App::new("do")
                .about("marks a task as complete")
                .arg(arg!(<ID> "The id of the task"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(
            App::new("undo")
                .about("marks a task as incomplete")
                .arg(arg!(<ID> "The id of the task"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(App::new("list").about("shows a list of tasks"))
}

pub fn run(arg_matches: &ArgMatches, config: &util::Config) -> Result<(), Box<dyn Error>> {
    match arg_matches.subcommand() {
        Some(("add", sub_matches)) => {
            let name = sub_matches
                .value_of("NAME")
                .expect("name argument not provided");
            let task = Task::new(name);
            let id = save(task, &config.conn)?;
            println!("{}", id);
            Ok(())
        }
        Some(("del", sub_matches)) => {
            let id = sub_matches
                .value_of("ID")
                .expect("id argument not provided");
            delete(id, &config.conn)?;
            println!("{}", id);
            Ok(())
        }
        Some(("do", sub_matches)) => {
            let id = sub_matches
                .value_of("ID")
                .expect("id argument not provided");

            complete(id, &config.conn)?;
            println!("{}", id);
            Ok(())
        }
        Some(("undo", sub_matches)) => {
            let id = sub_matches
                .value_of("ID")
                .expect("id argument not provided");

            undo(id, &config.conn)?;
            println!("{}", id);
            Ok(())
        }
        Some(("list", _)) => {
            let mut tasks: Vec<Task> = get_all(&config.conn)?;
            tasks.sort_by_key(|task| (task.done, task.name.to_string()));
            let output = tasks
                .iter()
                .map(task_to_string)
                .collect::<Vec<String>>()
                .join("\n");
            println!("{}", output);
            Ok(())
        }
        _ => {
            println!("Invalid subcommand");
            Ok(())
        }
    }
}

fn task_to_string(task: &Task) -> String {
    let mark = if task.done { "X" } else { " " };
    format!("[{2}] {0} {1}", task.id, task.name, mark)
}
