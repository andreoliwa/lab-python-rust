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
    // Initialize Python to avoid this error message:
    // thread 'main' panicked at /Users/aa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/pyo3-0.20.2/src/gil.rs:199:21:
    // assertion `left != right` failed: The Python interpreter is not initialized and the `auto-initialize` feature is not enabled.
    //
    // Consider calling `pyo3::prepare_freethreaded_python()` before attempting to use Python APIs.
    //   left: 0
    //  right: 0
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    pyo3::prepare_freethreaded_python();

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
