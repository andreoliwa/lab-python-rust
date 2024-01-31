use clap::{Parser, Subcommand};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    SlugRust { name: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::SlugRust { name } => slug_rust(name),
    }
}

fn slug_rust(name: String) -> Result<(), Box<dyn Error>> {
    Python::with_gil(|py| {
        let module = PyModule::import(py, "lab_python_rust")?;
        let rv: i32 = module.getattr("python_func_no_args")?.call0()?.extract()?;
        println!("return value: {}", rv);

        let args = (name,);
        let len: i32 = module
            .getattr("python_func_one_arg")?
            .call1(args)?
            .extract()?;
        println!("Length of slug: {}", len);

        let python_tuple = PyTuple::new(py, &["Itsy", "Bitsy", "Spider"]);
        let args = (python_tuple,);
        let len: i32 = module
            .getattr("python_func_tuple")?
            .call1(args)?
            .extract()?;
        println!("Length of slug: {}", len);

        let nested_module = PyModule::import(py, "lab_python_rust.utils")?;
        nested_module.getattr("func_in_nested_module")?.call0()?;

        Ok(())
    })
}
