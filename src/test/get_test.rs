#[cfg(test)]
mod test {
    use std::{env::set_current_dir, io::Error};

    use serial_test::serial;
    use tempfile::tempdir;

    use crate::{commands::{add::add_task, get::get_task}, utils::functions::{exist_dir}};

    #[test]
    #[serial]
    fn get_test() -> Result<(), Error> {
        let dir = tempdir()?;
        set_current_dir(&dir)?;
        exist_dir()?; 
        add_task("prueba".to_string(), "Unit testing".to_string())?;
        add_task("segunda prueba".to_string(), "Unit testing".to_string())?;

        assert!(get_task(1).is_ok(), "Error obteniendo la tarea");
        let task = get_task(2)?;
        assert_eq!(task.id, 2, "ID diferentes");
        assert_eq!(task.name,"segunda prueba", "Nombres diferentes");
        Ok(())
    }
}