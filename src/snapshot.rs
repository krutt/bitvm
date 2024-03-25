// SPDX-License-Identifier: MIT

use bitvm::vm::Snapshot as b_Snapshot;
use pyo3::pyclass;

#[pyclass]
pub struct Snapshot {
  virtual_machine: b_Snapshot
}
