// SPDX-License-Identifier: MIT

use pyo3::prelude::*;
use pyo3::{create_exception, exceptions::PyException};

create_exception!(bitvm, VirtualMachineError, PyException);

mod instruction;
mod merkle_path;
mod snapshot;
mod vm;

#[pymodule]
fn bitvm(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<instruction::Instruction>()?;
    m.add_class::<merkle_path::MerklePath>()?;
    m.add_class::<snapshot::Snapshot>()?;
    m.add_class::<vm::VM>()?;
    Ok(())
}
