use chrono::Local;
use std::io::{Error, ErrorKind};

use crate::utils::{data::Task, functions::{get_id, read_json_data, write_json_data}, status::StatusTask};

pub fn add_task(name: String, description: String) -> Result<u32,Error> {

    let t = Local::now().format("%d/%m/%Y %I:%M %p").to_string();

    let mut tasks_data = match read_json_data() {
        Ok(data) => data,
        Err(_) => return Err(Error::new(ErrorKind::NotFound,"Error al obtener las tareas: ruta no encontrada")), 
    };

    let id =  match get_id() {
        Ok(data) => data,
        Err(_) => return Err(Error::new(ErrorKind::Other,"Error al obtener el ID")), 
    };

    let new_task = Task {
        id: id,
        name: name,
        description: description,
        status: StatusTask::Pending,
        create: t,
        update: "".to_string(),
    };

    tasks_data.push(new_task);

    match write_json_data(&tasks_data) {
        Ok(_) => {}
        Err(_) => return Err(Error::new(ErrorKind::NotFound,"Error al actualizar las tareas")), 
    };

    Ok(id)

}