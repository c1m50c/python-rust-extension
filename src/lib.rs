extern crate cpython;


use cpython::{Python, PyResult, py_module_initializer, py_fn};


py_module_initializer!(pyrse, |py, m| {
    m.add(py, "__doc__", "Python Rust Extension")?;
    m.add(py, "pyrse_value", py_fn!(py, pyrse_value(value: &str)))?;
    m.add(py, "pyrse_count", py_fn!(py, pyrse_count(to: i64)))?;
    Ok(())
});


fn pyrse_value(_py: Python, value: &str) -> PyResult<String> {
    return Ok(format!("PyRsE: {}", value).to_owned());
}


fn pyrse_count(_py: Python, to: i64) -> PyResult<String> {
    let mut result: String = String::new();
    for i in 0 .. to { result.push_str(&i.to_string()); }
    return Ok(result);
}