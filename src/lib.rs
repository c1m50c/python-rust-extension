extern crate cpython;


use cpython::{Python, py_module_initializer, py_fn};
use cpython::{PyResult, PyList, PyString, PythonObject, ToPyObject};


py_module_initializer!(pyrse, |py, m| {
    m.add(py, "__doc__", "Python Rust Extension")?;
    m.add(py, "pyrse_value", py_fn!(py, pyrse_value(value: &str)))?;
    m.add(py, "pyrse_count", py_fn!(py, pyrse_count(to: i64)))?;
    Ok(())
});


fn pyrse_value(_py: Python, value: &str) -> PyResult<PyString> {
    return Ok(PyString::new(_py, format!("PyRsE: {}", value).as_ref()));
}


fn pyrse_count(_py: Python, to: i64) -> PyResult<PyList> {
    let result: PyList = PyList::new(_py, &[  ]);

    for i in 0 .. to {
        result.append(
            _py,
            i.to_py_object(_py).into_object()
        );
    }
    
    return Ok(result);
}