#[cfg(test)]
mod test {

    use std::{env::set_current_dir, io::Error};
    use tempfile::tempdir;
    use crate::{commands::{add::add_task, update::update_task}, 
    utils::{functions::{exist_dir,read_json_data}, status::StatusTask}};
    use serial_test::serial;

    #[test]
    #[serial]
    fn update_test() -> Result<(), Error> {
        let dir = tempdir()?;
        set_current_dir(&dir)?;

        exist_dir()?;
        add_task("prueba".to_string(), "Unit testing".to_string())?;
        let mut data = read_json_data()?;
        assert_eq!(data[0].name,"prueba", "Los nombres no son iguales");
        assert_eq!(data[0].description,"Unit testing", "Las descripciones no son iguales");
        assert_eq!(data[0].status,StatusTask::Pending, "Los estados son distintos");

        assert!(update_task(1, Some("nuevo nombre".to_string()),None, true).is_ok());
        data = read_json_data()?;
        assert_eq!(data[0].name,"nuevo nombre", "Los nombres no son iguales");
        assert_eq!(data[0].status,StatusTask::InProgress, "Los estados son distintos");

        assert!(update_task(1, None,None, true).is_ok());
        data = read_json_data()?;
        assert_eq!(data[0].status,StatusTask::Completed, "Los estados son distintos");
        Ok(())
    }
}