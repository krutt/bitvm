// SPDX-License-Identifier: MIT

use bitvm::ExecuteInfo as b_ExecuteInfo;
use pyo3::pyclass;

#[pyclass]
pub struct ExecuteInfo {
    information: b_ExecuteInfo,
}
