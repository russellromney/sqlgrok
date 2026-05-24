use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn parse_dialect(value: Option<&str>, default: sqlgrok::Dialect) -> PyResult<sqlgrok::Dialect> {
    match value {
        Some(value) => sqlgrok::Dialect::from_str(value)
            .ok_or_else(|| PyValueError::new_err(format!("unknown dialect: {value}"))),
        None => Ok(default),
    }
}

#[pyfunction]
#[pyo3(signature = (sql, read = None, write = None, pretty = false))]
fn transpile(
    sql: &str,
    read: Option<&str>,
    write: Option<&str>,
    pretty: bool,
) -> PyResult<Vec<String>> {
    let read = parse_dialect(read, sqlgrok::Dialect::Ansi)?;
    let write = parse_dialect(write, read)?;
    if pretty {
        sqlgrok::transpile_statements_pretty(sql, read, write)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    } else {
        sqlgrok::transpile_statements(sql, read, write)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }
}

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(transpile, module)?)?;
    Ok(())
}
