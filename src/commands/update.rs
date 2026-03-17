use chrono::Local;
use crate::utils::functions::{get_index, read_json_data, write_json_data};
use std::io::{Error, ErrorKind};

pub fn update_task(id:u32, name: Option<String>, description: Option<String>, change: bool) -> Result<(), Error> {

    let t = Local::now().format("%d/%m/%Y %I:%M %p").to_string();

    let mut tasks_data = match read_json_data() {
        Ok(data) => data,
        Err(_) => return Err(Error::new(ErrorKind::NotFound,"Error al obtener las tareas: ruta no encontrada")), 
    };

    let index = match get_index(&tasks_data, id) {
        Ok(data) => data,
        Err(_) => return Err(Error::new(ErrorKind::Other,"Tarea no encontrada")),
    };

    let data = &mut tasks_data[index as usize];
    data.update = t;

    match name {
        Some(n) => {
            println!("Nombre actualizada a: {n} ");
            data.name = n;}
        None => {}
    }

    match description {
        Some(d) => {
            println!("Descripción actualizada a: {d} ");
            data.description = d;}
        None => {}
    }

    if change && data.status.compare() != "completed"{
        let stat = data.status.next();
        data.status = stat;
        println!("Estado actualizado a: {stat} ");
    }

    match write_json_data(&tasks_data) {
        Ok(_) => {}
        Err(_) => return Err(Error::new(ErrorKind::NotFound,"Error al actualizar las tareas")), 
    };

    Ok(())

}