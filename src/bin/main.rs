use crate::helper::*;
use std::process;
use todo_rs::*;

fn main() {
    println!("Welcome to Todo app");
    let mut groups: Vec<Group> = Vec::new();

    loop {
        let command = prompt("command");
        let command: Vec<&str> = command.split(" ").collect();

        match command[0] {
            "create" => match command[1] {
                "group" => groups.push(Group::new(command[2].to_string())),
                "work" => add_work_in_group(&mut groups, command[2], command[3]),
                _ => println!("cant understand what's {}", command[1]),
            },
            "list" => match command[1] {
                "group" => list_group(&groups),
                "work" => list_work(&groups, command[2]),
                "all" => list_all(&groups),
                _ => continue,
            },
            "remove" => match command[1] {
                "group" => println!("list group"),
                "work" => println!("list work"),
                "all" => println!("list everything"),
                _ => println!("please specify what to delete"),
            },
            "exit" => {
                println!("bye bye");
                process::exit(1)
            }
            _ => println!("wrong command, try help or h"),
        }
    }
}
