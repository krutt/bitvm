// SPDX-License-Identifier: MIT

use bitvm::vm::MerklePath as b_MerklePath;
use pyo3::pyclass;

#[pyclass]
pub struct MerklePath {
  merkle_path: b_MerklePath
}
