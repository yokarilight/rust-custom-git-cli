use clap::{Arg, Command};
use std::process::{Command as ProcessCommand, Stdio};

fn main() {
    let matches = Command::new("custom-git-cli")
        .version("1.0")
        .about("A custom Git CLI tool with simplified git commands")
        .subcommand(Command::new("gf").about("Run git fetch"))
        .subcommand(Command::new("gp").about("Run git pull"))
        .subcommand(Command::new("gfp").about("Run git fetch && git pull"))
        .subcommand(Command::new("gs").about("Run git status"))
        .subcommand(Command::new("gsu").about("Run git stash -u"))
        .subcommand(Command::new("gstash").about("Run git stash pop with index").arg(Arg::new("index").required(true)).arg(Arg::new("shift").long("shift")))
        .subcommand(Command::new("gl").about("Run git log").arg(Arg::new("oneline").long("oneline")))
        .subcommand(Command::new("gch").about("Run git checkout <branch>").arg(Arg::new("branch").required(true)))
        .subcommand(Command::new("gcom").about("Run git commit -m <message>").arg(Arg::new("message").required(true)))
        .subcommand(Command::new("gpush").about("Run git push origin <branch>").arg(Arg::new("branch").required(true)))
        .get_matches();

    match matches.subcommand() {
        Some(("gf", _)) => run_git_command(&["fetch"]),
        Some(("gp", _)) => run_git_command(&["pull"]),
        Some(("gfp", _)) => {
            run_git_command(&["fetch"]);
            run_git_command(&["pull"]);
        }
        Some(("gs", _)) => run_git_command(&["status"]),
        Some(("gsu", _)) => run_git_command(&["stash", "-u"]),
        Some(("gstash", sub_m)) => {
            let index = sub_m.get_one::<String>("index").unwrap();
            let stash_command = format!("stash@{{{}}}", index);
            run_git_command(&["stash", "pop", &stash_command]);
        }
        Some(("gl", sub_m)) => {
            if sub_m.contains_id("oneline") {
                run_git_command(&["log", "--oneline"]);
            } else {
                run_git_command(&["log"]);
            }
        }
        Some(("gch", sub_m)) => {
            let branch = sub_m.get_one::<String>("branch").unwrap();
            run_git_command(&["checkout", branch]);
        }
        Some(("gcom", sub_m)) => {
            let message = sub_m.get_one::<String>("message").expect("Commit message is required");
            run_git_command(&["commit", "-m", message]);
        }
        Some(("gpush", sub_m)) => {
            let branch = sub_m.get_one::<String>("branch").expect("Branch name is required");
            run_git_command(&["push", "origin", branch]);
        }
        _ => eprintln!("Unknown command!"),
    }
}

fn run_git_command(args: &[&str]) {
    let output = ProcessCommand::new("git")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute git command");

    if !output.status.success() {
        eprintln!("Error: Git command failed with status {:?}", output.status);
    }
}
