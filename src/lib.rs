use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use uiua::*;

fn format_value(value: &Value) -> String {
    match value {
        Value::Byte(array) => format!("{:?}", array),
        Value::Num(array) => format!("{:?}", array),
        Value::Complex(array) => format!("{:?}", array),
        Value::Char(array) => format!("{:?}", array),
        Value::Box(array) => format!("{:?}", array),
    }
}

#[pyfunction]
fn run_file(path: &str) -> PyResult<String> {
    let mut uiua = Uiua::with_native_sys();
    let _ = uiua.compile_run(|comp| {
        comp.print_diagnostics(true).load_file(path)
    });

    let stack = uiua.stack();

    if stack.len() > 1 {
        let formatted_stack: Vec<String> = stack.iter().map(|value| format_value(value)).collect();
        Ok(formatted_stack.join("\n"))  // Объединяем элементы стека через запятую
    } else if let Some(value) = stack.last() {
        Ok(format_value(value))
    } else {
        Err(PyRuntimeError::new_err("Stack is empty"))
    }
}

#[pyfunction]
fn run_code(input: &str) -> PyResult<String> {
    let mut uiua = Uiua::with_native_sys();
    let _ = uiua.compile_run(|comp| {
        comp.print_diagnostics(true).load_str(input)
    });

    let stack = uiua.stack();

    if stack.len() > 1 {
        let formatted_stack: Vec<String> = stack.iter().map(|value| format_value(value)).collect();
        Ok(formatted_stack.join("\n"))  // Объединяем элементы стека через запятую
    } else if let Some(value) = stack.last() {
        Ok(format_value(value))
    } else {
        Err(PyRuntimeError::new_err("Stack is empty"))
    }
}

#[pymodule]
fn uiua_interpreter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_code, m)?)?;
    m.add_function(wrap_pyfunction!(run_file, m)?)?;
    Ok(())
}

