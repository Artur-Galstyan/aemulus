use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn start_sim_server(device_name: String, device_id: String, port: u16) -> PyResult<()> {
    let _ = tracing_subscriber::fmt::try_init();
    if let Err(e) = aemulus_servers::start_sim_server(device_name, device_id, port) {
        return Err(PyValueError::new_err(e));
    }

    Ok(())
}

#[pymodule]
#[pyo3(name = "_native")]
fn native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_sim_server, m)?)?;
    Ok(())
}
