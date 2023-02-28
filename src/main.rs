use std::path::{PathBuf};

use pyembed::OxidizedPythonInterpreterConfig;


fn main() {

    let config = OxidizedPythonInterpreterConfig {
        exe: Some(PathBuf::from("/Users/drunisa/python_test/using_py_from_rs")),
        ..Default::default()
    };
    // let current_path =  Path::new("/Users/drunisa/python_test/using_py_from_rs");
    // config.exe = Some(current_path.to_owned());
    // println!("{:?}",config.exe);

   let interpreter =  pyembed::MainPythonInterpreter::new(config).unwrap();

   interpreter.with_gil(|py| {
    match py.eval("print('hello, world')",None,None) {
       Ok(_) => print!("python code executed successfully"),
       Err(e) => print!("python error: {:?}", e),
   }
});
}
