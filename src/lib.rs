mod watcher;

extern crate notify;
extern crate pyo3;

use pyo3::prelude::*;
use crate::watcher::{Watcher, WatcherError};

#[pymodule]
fn _inotify_toolkit_lib(py: Python, m: &PyModule) -> PyResult<()> {
    let mut version = env!("CARGO_PKG_VERSION").to_string();
    version = version.replace("-alpha", "a").replace("-beta", "b");

    m.add("__version__", version)?;

    m.add("WatcherError", py.get_type::<WatcherError>())?;

    m.add_class::<Watcher>()?;

    Ok(())
}
