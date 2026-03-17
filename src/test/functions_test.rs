#[cfg(test)]
mod test {
    use crate::utils::{data::Task, functions::*, status::StatusTask};    
    use chrono::Local;
    use tempfile::tempdir;
    use serial_test::serial;
    use std::{env::set_current_dir, io::Error};

    #[test]
    #[serial]
    fn test_exist_dir() -> Result<(), Error> {
        let dir = tempdir()?;
        set_current_dir(&dir)?;
        assert!(exist_dir().is_ok());
        Ok(())
    }

    #[test]
    #[serial]
    fn test_read_json() -> Result<(), Error> {
        let dir = tempdir()?;
        set_current_dir(&dir)?;
        exist_dir()?;
        assert!(read_json_data().is_ok());
        Ok(())
    }

    #[test]
    #[serial]
    fn test_write_json() -> Result<(), Error> {
        let dir = tempdir()?;
        set_current_dir(&dir)?;
        exist_dir()?;
        let task = Task {
            id: 1,
            name: "prueba".to_string(),
            description: "descripción".to_string(),
            status: StatusTask::Pending,
            create: Local::now().format("%d/%m/%Y %I:%M %p").to_string(),
            update: "".to_string()
        };
        assert!(write_json_data(&vec![task]).is_ok());
        Ok(())
    }

    #[test]
    #[serial]
    fn test_get_id() -> Result<(), Error> {
        let dir = tempdir()?;
        set_current_dir(&dir)?;
        exist_dir()?;
        assert!(get_id().is_ok(), "Error al conseguir el id");
        assert_eq!(get_id()?,2,"Error en el id");
        Ok(())
    }

    #[test]
    #[serial]
    fn test_get_index() -> Result<(), Error> {
        let dir = tempdir()?;
        set_current_dir(&dir)?;
        exist_dir()?;
        let task = Task {
            id: 5,
            name: "prueba".to_string(),
            description: "descripción".to_string(),
            status: StatusTask::Pending,
            create: Local::now().format("%d/%m/%Y %I:%M %p").to_string(),
            update: "".to_string()
        };
        write_json_data(&vec![task])?;
        let data = read_json_data()?;
        assert!(get_index(&data,5).is_ok(), "Error al conseguir el index");
        assert_eq!(get_index(&data,5)?,0,"Error al conseguir el index");
        Ok(())
    }

}