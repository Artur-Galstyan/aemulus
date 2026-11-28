use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn start_sim_server(server_name: String, device_name: String) -> PyResult<()> {
    let _ = tracing_subscriber::fmt::try_init();
    match server_name.as_str() {
        "pf400" => {
            std::thread::spawn(|| {
                let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
                if let Err(err) = rt.block_on(
                    aemulus_servers::devices::pf400::sim::serve_pf400_sim(device_name),
                ) {
                    tracing::error!("pf400 sim server failed: {err}");
                }
            });
            Ok(())
        }
        "starlet" => {
            std::thread::spawn(|| {
                let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
                if let Err(err) = rt.block_on(
                    aemulus_servers::devices::starlet::sim::serve_starlet_sim(device_name),
                ) {
                    tracing::error!("starlet sim server failed: {err}");
                }
            });
            Ok(())
        }
        other => Err(PyValueError::new_err(format!(
            "unknown sim server: {other}"
        ))),
    }
}

#[pymodule]
#[pyo3(name = "_native")]
fn native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_sim_server, m)?)?;
    Ok(())
}
