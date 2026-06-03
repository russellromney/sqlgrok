use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

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

#[pyfunction]
fn transpile_many(py: Python<'_>, requests: Vec<Bound<'_, PyDict>>) -> PyResult<Py<PyList>> {
    let mut rust_requests = Vec::with_capacity(requests.len());
    for request in requests {
        let sql: String = request
            .get_item("sql")?
            .ok_or_else(|| PyValueError::new_err("request is missing 'sql'"))?
            .extract()?;
        let read: Option<String> = request
            .get_item("read")?
            .map(|value| value.extract())
            .transpose()?;
        let write: Option<String> = request
            .get_item("write")?
            .map(|value| value.extract())
            .transpose()?;
        let pretty: bool = request
            .get_item("pretty")?
            .map(|value| value.extract())
            .transpose()?
            .unwrap_or(false);

        let read = parse_dialect(read.as_deref(), sqlgrok::Dialect::Ansi)?;
        let write = parse_dialect(write.as_deref(), read)?;
        rust_requests.push(sqlgrok::TranspileRequest {
            sql,
            read,
            write,
            pretty,
        });
    }

    let rows = PyList::empty(py);
    for result in sqlgrok::transpile_many(&rust_requests) {
        let row = PyDict::new(py);
        row.set_item("ok", result.ok)?;
        row.set_item("sql", result.sql)?;
        row.set_item("error", result.error)?;
        rows.append(row)?;
    }
    Ok(rows.into())
}

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(transpile, module)?)?;
    module.add_function(wrap_pyfunction!(transpile_many, module)?)?;
    Ok(())
}
