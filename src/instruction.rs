// SPDX-License-Identifier: MIT

use bitvm::vm::Instruction as b_Instruction;
use pyo3::pyclass;

#[pyclass]
pub struct Instruction {
  instruction: b_Instruction
}
