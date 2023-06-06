use console::{style, Key, Term};
use ctrlc;
use clearscreen::clear;
use std::sync::mpsc;
use std::time;
use std::{
    io::stdin,
    process::{exit, Command, Stdio},
};

fn run_command(command: &mut String, arg: Vec<String>) -> bool { 
    let term = Term::stdout();
    //term.clear_screen().unwrap();
    clear().unwrap();

    let mut title = String::new();
    title += "  [";
    title += command;
    title += "] run tool 1.0";
    for _ in 0..(term.size().1 - (command.len() as u16) - 3 - 14) {
        title += " ";
    }

    println!("{}", style(title).on_blue());

    let timer = time::Instant::now();
    let mut task = match Command::new("cmd")
        .arg("/c")
        .arg(&command)
        .args(&arg)
        .stderr(Stdio::piped()).spawn() {
        Ok(c) => c,
        Err(_c) => {
            println!("{}", style("wrong command").red());
            println!(
                "{}",
                style(format!(
                    "\n > command: {} {:?}",
                    &command,
                    &arg
                ))
                .green()
            );
            return true
        }
    };
    let (tx, rx) = mpsc::channel::<bool>();
    let ctrlc_handler = move || {
        tx.send(true).expect("send message failed");
    };
    match ctrlc::set_handler(ctrlc_handler) {
        _ => {}
    }
    let exit_code;
    let mut is_killed = false;
    loop {
        match task.try_wait() {
            Ok(Some(statue)) => {
                exit_code = statue;
                break;
            }
            _ => {}
        }
        match rx.try_recv().is_ok() {
            true => {
                task.kill().unwrap();
                is_killed = true;
            },
            _ => {}
        }
    }
    if is_killed {
        println!("{}",
            style(format!("\n > process killed in {}s\n command: {} {:?}",
                timer.elapsed().as_secs(),
                &command,
                &arg
            ))
            .green()
        );
    } else {
        println!(
            "{}",
            style(format!(
                "\n > {} run in {}s\n > command: {} {:?}",
                exit_code,
                timer.elapsed().as_secs(),
                &command,
                &arg
            ))
            .green()
        );
    }
    loop {
        match term.read_key().unwrap() {
            Key::Escape => {
                term.clear_screen().unwrap();
                exit(0);
            }
            Key::Enter => break,
            Key::Char('r') => return true,
            _ => {}
        }
    }
    false
}

#[warn(unused_variables)]
fn main() {
    let mut command = String::from("cmd");
    let mut arg = vec![String::from("hello")];
    loop {
        // 重命名命令
        let mut input = String::new();

        if run_command(&mut command, arg.clone()) {

            println!("{}", style("\n reset command: ").yellow());
            stdin()
                .read_line(&mut input)
                .expect("read command failed");
            
            let collection: Vec<_> = input.split_whitespace().map(|s| s.to_string()).collect();
            command = collection[0].clone();
            if command == String::from("quit") {
                break;
            }
            arg = collection[1..].to_vec().clone();
        }
    }
}
