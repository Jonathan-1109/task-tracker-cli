mod commands;
mod utils;

use commands::cli_commands::{Cli, Commands};
use utils::functions::{exist_dir};
use clap::Parser;

use crate::{commands::{add::add_task, delete::delete_task, get::get_task, list::list_tasks, update::update_task}, 
utils::functions::print_table};

fn main() {
    match exist_dir() {
      Ok(()) => {},
      Err(_) => eprintln!("Error al crear el archivo"),
    };
    let temp = Cli::parse();
    match temp.command {

        Commands::Add { name, description } => {
            let desc = description.unwrap_or("".to_string());
            match add_task(name, desc) {
                Ok(id) => println!("Tarea añadida, id: {}", id),
                Err(e) => eprintln!("{e}"),
            };
        },

        Commands::Delete { id } => {
            let id_int = match id.parse::<u32>() {
                Ok(_) => id.parse::<u32>().unwrap(),
                Err(_) => { eprintln!("ID con formato invalido: utilize numeros enteros positivos"); return; }
            };
            match delete_task(id_int) {
                Ok(_) => println!("Tarea eliminada correctamente"),
                Err(e) => eprintln!("{e}"),
            };
        },

        Commands::Get { id } => {
            
            let id_int = match id.parse::<u32>() {
                Ok(_) => id.parse::<u32>().unwrap(),
                Err(_) => { eprintln!("ID con formato invalido: utilize numeros enteros positivos"); return; }
            };
            match get_task(id_int) {
                Ok(data) => print_table(vec![data]),
                Err(e) => eprintln!("{e}"), 
            };
        },

        Commands::Update { id, description, name , change} => {
            let c: bool = change.unwrap_or(true);
            let id_int = match id.parse::<u32>() {
                Ok(_) => id.parse::<u32>().unwrap(),
                Err(_) => { eprintln!("ID con formato invalido: utilize numeros enteros positivos"); return; }
            };

            match update_task(id_int, name, description, c) {
                Ok(_) => {},
                Err(e) => eprintln!("{e}"),    
            }; 
        }
        
        Commands::List { mode } => {
            match list_tasks(mode) {
                Ok(data) => print_table(data),
                Err(e) => eprintln!("{e}"), 
            };
        }
    }

}
