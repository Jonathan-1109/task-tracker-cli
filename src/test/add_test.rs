#[cfg(test)]
mod test {

    use std::{env::set_current_dir, io::Error};
    use tempfile::tempdir;
    use crate::{commands::add::add_task, utils::{functions::{exist_dir, read_json_data}}};
    use serial_test::serial;

    #[test]
    #[serial]
    fn add_test() -> Result<(), Error> {
        let dir = tempdir()?;
        set_current_dir(&dir)?;

        exist_dir()?;
        assert!(add_task("prueba".to_string(), "Unit testing".to_string()).is_ok(), "Error al añadir tareas");
        assert!(add_task("segunda prueba".to_string(), "Unit testing".to_string()).is_ok(), "Error al añadir tareas");

        let data = read_json_data()?;
        assert_eq!(data[0].name,"prueba", "Los nombres no son iguales");
        assert_eq!(data[1].id,2,  "Los ID son distintos");
        assert_ne!(data[0].id,2, "Los ID son iguales");
        Ok(())
    }
}