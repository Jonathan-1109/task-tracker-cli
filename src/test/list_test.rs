#[cfg(test)]
mod test {
    use std::{env::set_current_dir, io::Error};

    use serial_test::serial;
    use tempfile::tempdir;

    use crate::{commands::{add::add_task, get::get_task, 
    list::list_tasks, update::update_task}, utils::functions::exist_dir};

    #[test]
    #[serial]
    fn list_test() -> Result<(), Error> {
        let dir = tempdir()?;
        set_current_dir(&dir)?;
        exist_dir()?; 
        add_task("prueba".to_string(), "Unit testing".to_string())?;
        add_task("segunda prueba".to_string(), "Unit testing".to_string())?;
        add_task("tercera prueba".to_string(), "Unit testing".to_string())?;
        add_task("cuarta prueba".to_string(), "Unit testing".to_string())?;
        update_task(1, None, None, true)?;
        update_task(1, None, None, true)?;
        update_task(2, None, None, true)?;
        update_task(4, None, None, true)?;

        assert_eq!(list_tasks("all".to_string())?.len(), 4, "Error obteniendo todos los valores");
        assert_eq!(list_tasks("inprogress".to_string())?.len(), 2, "Error obteniendo todos los valores");
        assert_eq!(list_tasks("completed".to_string())?.len(), 1, "Error obteniendo todos los valores"); 
        assert_eq!(list_tasks("pending".to_string())?.len(), 1, "Error obteniendo todos los valores"); 
        let task = get_task(3)?;
        let task_pending = list_tasks("pending".to_string())?[0].clone();
        assert_eq!(task, task_pending, "No son iguales las tareas");
        Ok(())
    }
}