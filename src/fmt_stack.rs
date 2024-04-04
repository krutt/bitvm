// SPDX-License-Identifier: MIT

use bitvm::FmtStack as b_FmtStack;
use pyo3::pyclass;

#[pyclass]
pub struct FmtStack {
  stack: b_FmtStack
}
