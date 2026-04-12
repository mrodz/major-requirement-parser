use libmql::{parse as libmql_parse, parse_with_externals as libmql_parse_with_externals, parse_extern_value as libmql_parse_extern_value, ParseResult};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::collections::HashMap;

#[pyclass]
pub struct MQL {
    result: ParseResult,
}

#[pymethods]
impl MQL {
    fn version(&self) -> &'static str {
        self.result.parsed_mql_file().version()
    }

    fn json(&self) -> PyResult<String> {
        self.result
            .to_string()
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e:?}")))
    }

    fn json_pretty(&self) -> PyResult<String> {
        self.result
            .to_string_pretty()
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e:?}")))
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.result)
    }
}

#[pyfunction]
fn parse(mql: String) -> PyResult<MQL> {
    let result = libmql_parse(&mql)
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e:?}")))?;
    Ok(MQL { result })
}

#[pyfunction]
fn parse_with_externals(mql: String, externals: HashMap<String, String>) -> PyResult<MQL> {
    // Convert the HashMap<String, String> to HashMap<String, VarValue>
    let mut extern_vars = HashMap::new();
    for (key, value_str) in externals {
        let var_value = libmql_parse_extern_value(&value_str)
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("Error parsing extern value for {}: {:?}", key, e)))?;
        extern_vars.insert(key, var_value);
    }

    let result = libmql_parse_with_externals(&mql, extern_vars)
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e:?}")))?;
    Ok(MQL { result })
}

#[pyfunction]
fn parse_extern_value(expr: String) -> PyResult<String> {
    let var_value = libmql_parse_extern_value(&expr)
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e:?}")))?;
    Ok(format!("{:?}", var_value))
}

#[pymodule]
fn pylibmql(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(parse_with_externals, m)?)?;
    m.add_function(wrap_pyfunction!(parse_extern_value, m)?)?;
    m.add_class::<MQL>()?;
    Ok(())
}
