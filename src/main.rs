//use std::{env, path::PathBuf};

use std::path::Path;

use pyembed::OxidizedPythonInterpreterConfig;

//use pyo3::{PyResult, Python, types::PyList};
fn main() {

    let mut config = OxidizedPythonInterpreterConfig::default();

    let current_path =  Path::new("/Users/drunisa/python_test/using_py_from_rs");
    config.exe = Some(current_path.to_owned());
    println!("{:?}",config.exe);

    let interpreter = pyembed::MainPythonInterpreter::new(config).unwrap();

    // interpreter.with_gil(|py| match py.eval("print('hello, world')", None, None) {
    //     Ok(_) => print!("python code executed successfully"),
    //     Err(e) => print!("python error: {:?}", e),
    // });

    // Python::with_gil(|py| {
    //     let sys = py.import("sys")?;
    //     let path:&PyList = sys.getattr("path")?.extract()?;
    //     print!("{:?}",path);
    //     path.insert(0, "/path/to/single_file_executable")?;
    //     print!("{:?}",path);
    //     let my_module = py.import("main")?;
    //     let result = my_module.getattr("main")?;
    //     println!("{:?}",result.call0());
    //     Ok(())
    // })
 
}
