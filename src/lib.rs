// SPDX-License-Identifier: MIT

use pyo3::prelude::*;
use pyo3::{create_exception, exceptions::PyException};

create_exception!(bitvm, VirtualMachineError, PyException);

mod execute_info;
mod fmt_stack;

#[pymodule]
fn bitvm(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ExecuteInfo>()?;
    m.add_class::<FmtStack>()?;
    Ok(())
}
