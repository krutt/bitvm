// SPDX-License-Identifier: MIT

use bitvm::vm::VM as b_VM;
use pyo3::pyclass;

#[pyclass]
pub struct VM {
  virtual_machine: b_VM
}
