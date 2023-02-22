use pyo3::prelude::*;
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

    Python::with_gil(|py| {
        let builtins = PyModule::import(py, "builtins")?;
        let total: i32 = builtins
            .getattr("sum")?
            .call1((vec![1, 2, 3],))?
            .extract()?;
        assert_eq!(total, 6);
        println!("{}",total);
        Ok(())
    })
}
