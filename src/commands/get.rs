use crate::utils::{data::Task, functions::{get_index, read_json_data}};
use std::io::{Error, ErrorKind};


pub fn get_task(id: u32) -> Result<Task, Error> {
    let tasks_data = match read_json_data() {
        Ok(data) => data,
        Err(_) => return Err(Error::new(ErrorKind::NotFound, "Error al obtener las tareas: ruta no encontrada")),
    };

    let index = match get_index(&tasks_data, id) {
        Ok(data) => data,
        Err(_) => return Err(Error::new(ErrorKind::Other, "Tarea no encontrada")),
    };

    Ok(tasks_data[index as usize].clone())
}