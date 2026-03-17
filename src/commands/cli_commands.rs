use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Añade tareas nuevas")]
    Add {
        #[arg(help = "Nombre de la tarea que deseas agregar")]
        name: String,
        #[arg(short = 'd', long = "description", help="Agregar descripción a la tarea")]
        description: Option<String>,
    },
    #[command(about = "Elimina tareas existentes")]
    Delete {
        #[arg(help = "ID de la tarea a eliminar")]
        id: String
    },
    #[command(about = "Buscar una tarea")]
    Get {
        #[arg(help = "ID de la tarea a buscar")]
        id: String
    },
    #[command(about = "Actualiza una tarea")]
    Update {
        #[arg(help = "ID de la tarea a buscar")]
        id: String,
        #[arg(short = 'd', long = "description", help="Agregar descripción a la tarea")]
        description: Option<String>,
        #[arg(short = 'n', long = "name", help="Cambiar nombre a la tarea")]
        name: Option<String>,
        #[arg(short = 'c', long = "change", help="Actualiza al siguiente estado")]
        change: Option<bool>,
    },
    List {
        #[arg(help = "Modo a listar (all, pending, inprogress, completed)")]
        mode: String
    }

}
