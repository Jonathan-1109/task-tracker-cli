#[cfg(test)]
mod test {
    use std::{env::set_current_dir, io::Error};

    use serial_test::serial;
    use tempfile::tempdir;

    use crate::{commands::{add::add_task, delete::delete_task}, utils::functions::{exist_dir, read_json_data}};

    #[test]
    #[serial]
    fn delete_test() -> Result<(), Error> {
        let dir = tempdir()?;
        set_current_dir(&dir)?;
        exist_dir()?;        
        add_task("prueba".to_string(), "Unit testing".to_string())?;
        add_task("segunda prueba".to_string(), "Unit testing".to_string())?;
        add_task("Tercero".to_string(), "Unit testing".to_string())?;
        assert!(delete_task(1).is_ok());
        let data = read_json_data()?;
        assert_ne!(data[0].id,0, "ID inexistente");
        assert_eq!(data[1].id,3,"Error: Tarea inexistente");
        Ok(())
    } 
}