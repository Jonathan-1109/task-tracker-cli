use crate::utils::functions::{get_index, read_json_data, write_json_data};
use std::io::{Error, ErrorKind};

pub fn delete_task(id: u32) -> Result<(), Error> {

    let mut tasks_data = match read_json_data() {
        Ok(data) => data,
        Err(_) => return Err(Error::new(ErrorKind::NotFound,"Error al obtener las tareas: ruta no encontrada")), 
    };

    let index = match get_index(&tasks_data, id) {
        Ok(data) => data,
        Err(_) => return Err(Error::new(ErrorKind::Other,"Tarea no encontrada")),
    };

    tasks_data.remove(index as usize);

    match write_json_data(&tasks_data) {
        Ok(_) => {}
        Err(_) => return Err(Error::new(ErrorKind::NotFound,"Error al actualizar las tareas")), 
    };
    Ok(())

}