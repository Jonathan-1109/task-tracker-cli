use std::fs::{self, File};
use std::io::{Error};
use std::vec;
use fs::{read_to_string};
use prettytable::{Cell, Row, Table};
use crate::utils::data::{NextID, Task};

pub fn exist_dir() -> Result<(), Error> {
    let dir = std::env::current_dir()?;
    let tasks_dir = dir.join("tasks");
    let file_name = tasks_dir.join("data.json");
    let file_config = tasks_dir.join("config.json");

    if !tasks_dir.exists()  {
        fs::create_dir_all(&tasks_dir)?;
        File::create(&file_name)?;

        let next_id = NextID { next: 1 };
        let new_json = serde_json::to_string_pretty(&next_id)?;
        fs::write(&file_config, new_json)?;
        
    }

    if !file_name.exists() {
        File::create(&file_name)?;
    }
    

    if !file_config.exists() {
        let next_id = NextID { next: 1 };
        let new_json = serde_json::to_string_pretty(&next_id)?;
        fs::write(&file_config, new_json)?;
    }

    Ok(())
}

pub fn read_json_data() -> Result<Vec<Task>, Error> {
    let dir = std::env::current_dir()?;
    let file_name = dir.join("tasks/data.json");
    let json_data= read_to_string(&file_name)?;

    if json_data.is_empty() {
        return Ok(Vec::new());
    }
    let tasks_data: Vec<Task> = serde_json::from_str(&json_data)?;

    Ok(tasks_data)
}

pub fn write_json_data(task_data:  &Vec<Task>) -> Result<(), Error> {
    let dir = std::env::current_dir()?;
    let file_name = dir.join("tasks/data.json");
    fs::write(&file_name, serde_json::to_string_pretty(&task_data)?)?;
    Ok(())
}

pub fn get_id() -> Result<u32, Error> {
    let dir = std::env::current_dir()?;
    let file_name = dir.join("tasks/config.json");
    let json_data= read_to_string(&file_name)?;
    let id: NextID = serde_json::from_str(&json_data)?;
    let new_json = NextID {next: id.next + 1 };
    fs::write(&file_name, serde_json::to_string_pretty(&new_json)?)?;
    Ok(id.next)
}

pub fn get_index(task_data:  &Vec<Task>, id: u32) -> Result<u32, Error> {
    let mut index: u32 = 0;
    for element in task_data {
        if element.id == id {
            return Ok(index)
        }
        index += 1;
    }
    return Err(Error::other("Index no encontrado"));
}

pub fn print_table(vector: Vec<Task>) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID"),Cell::new("Nombre"), Cell::new("Descripción"), Cell::new("Estado"), Cell::new("Creación"), 
        Cell::new("Actualización") ]));
    
    for task in vector {
        table.add_row(Row::new(vec![
            Cell::new(&task.id.to_string()), Cell::new(&task.name), Cell::new(&task.description), 
            Cell::new(&task.status.to_string()), Cell::new(&task.create), Cell::new(&task.update), 
        ]));
    }
    table.printstd()
}