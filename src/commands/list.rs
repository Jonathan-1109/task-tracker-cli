use std::io::{Error, ErrorKind};

use crate::utils::{data::Task, functions::read_json_data};

pub fn list_tasks(mode: String) -> Result<Vec<Task>, Error> {
    let status_avalible = vec!["all", "pending", "inprogress", "completed"];
    let mut copy_task_data = vec![];

    if !status_avalible.contains(&mode.to_lowercase().as_str()) {
        return Err(Error::new(ErrorKind::Other,
            "Error: modo de lista invalido, utilize: (all, pending, inprogress, completed)"
        ));
    }

    let tasks_data = match read_json_data() {
        Ok(data) => data,
        Err(_) => return Err(Error::new(ErrorKind::NotFound,"Error al obtener las tareas: ruta no encontrada")), 
    };

    for task in tasks_data {
        if mode.to_lowercase() == "all" || mode.to_lowercase() == task.status.compare()  {
            copy_task_data.push(task);
        }
    }
    Ok(copy_task_data)

}