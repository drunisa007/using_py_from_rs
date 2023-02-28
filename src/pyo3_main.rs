use std::{fs, path::Path};

use pyo3::{prelude::*, types::PyList};
//use pyo3::types::IntoPyDict;
//use pyo3::types::PyTuple;

fn main() -> PyResult<()> {
    // Python::with_gil(|py| {
    //     // let sys = py.import("sys")?;

    //     // let version: String = sys.getattr("version")?.extract()?;

    //     // let locals = [("os", py.import("os")?)].into_py_dict(py);
    //     // let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
    //     // let user: String = py.eval(code, None, Some(&locals))?.extract()?;

    //     // println!("Hello {}, I'm Python {}", user, version);
    //     // Ok(())
    // })

    // let arg1 = "arg1";
    // let arg2 = "arg2";
    // let arg3 = "arg3";

    // Python::with_gil(|py| {
    //     let fun: Py<PyAny> = PyModule::from_code(
    //         py,
    //         "def example(*args, **kwargs):
    //             if args != ():
    //                 print('called with args', args)
    //             if kwargs != {}:
    //                 print('called with kwargs', kwargs)
    //             if args == () and kwargs == {}:
    //                 print('called with no arguments')",
    //         "",
    //         "",
    //     )?
    //     .getattr("example")?
    //     .into();

    //     // call object without any arguments
    //     fun.call0(py)?;

    //     // call object with PyTuple
    //     let args = PyTuple::new(py, &[arg1, arg2, arg3]);
    //     fun.call1(py, args)?;

    //     // pass arguments as rust tuple
    //     let args = (arg1, arg2, arg3);
    //     fun.call1(py, args)?;
    //     Ok(())
    // })

    // Python::with_gil(|py| {
    //     let builtins = PyModule::import(py, "builtins")?;
    //     let total: i32 = builtins
    //         .getattr("sum")?
    //         .call1((vec![1, 2, 3],))?
    //         .extract()?;
    //     assert_eq!(total, 6);
    //     println!("{}",total);
    //     Ok(())
    // })

    // let py_foo = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/utils/foo.py"));
    // let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/app.py"));
    // let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
    //     PyModule::from_code(py, py_foo, "utils.foo", "utils.foo")?;
    //     let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
    //         .getattr("run")?
    //         .into();
    //     app.call0(py)
    // });

    // println!("py: {}", from_python?);

    let path = Path::new("/Users/drunisa/python_test/using_py_from_rs/python_app");
    let py_app = fs::read_to_string(path.join("app.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("run")?
            .into();
        app.call0(py)
    });

    println!("py: {}", from_python?);
    Ok(())
}
