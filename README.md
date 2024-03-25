# BitVM

[![Package vesion](https://img.shields.io/pypi/v/bitvm)](https://pypi.org/project/bitvm)
[![Format](https://img.shields.io/pypi/format/bitvm)](https://pypi.org/project/bitvm)
[![Python version](https://img.shields.io/pypi/pyversions/bitvm)](https://pypi.org/project/bitvm)
[![License](https://img.shields.io/pypi/l/bitvm)](https://pypi.org/project/bitvm)
[![Code size](https://img.shields.io/github/languages/code-size/krutt/bitvm)](.)
[![Top](https://img.shields.io/github/languages/top/krutt/bitvm)](.)
[![Languages](https://img.shields.io/github/languages/count/krutt/bitvm)](.)
[![Repository size](https://img.shields.io/github/repo-size/krutt/bitvm)](.)
[![Last commit](https://img.shields.io/github/last-commit/krutt/bitvm/master)](.)

![BitVM banner](https://github.com/krutt/bitvm/blob/master/static/bitvm-banner.svg)

Vulpes Macrotis, henceforth VM, is better known as Kit Fox or Kitsune. BitVM is the brainchild of 
the amazing people at ZeroSync and this is only an attempted Python library using Rust bidings
available via [PyO3](https://github.com/PyO3/pyo3) package.

## Contributions

To contribute to the project, fork the repository and clone to your local device and development
dependencies including four extra libraries not included in final builds as such:

- **maturin** Build and publish crates with pyo3, rust-cpython and cffi bindings as well as rust binaries as python packages
  [![GitHub](https://img.shields.io/badge/GitHub-2B3137?logo=github&logoColor=white)](https://github.com/PyO3/maturin)
  [![PyPI](https://img.shields.io/badge/-PyPI:%20maturin-3775A9?logo=pypi&logoColor=white)](https://pypi.org/project/maturin)
  [![Docs](https://img.shields.io/badge/user-guide-brightgreen?logo=readthedocs)](https://maturin.rs)
- **mypy** Optional static typing for Python
  [![GitHub](https://img.shields.io/badge/GitHub-2B3137?logo=github&logoColor=white)](https://github.com/python/mypy)
  [![PyPI](https://img.shields.io/badge/-PyPI:%20mypy-3775A9?logo=pypi&logoColor=white)](https://pypi.org/project/mypy)
  [![Docs](https://img.shields.io/readthedocs/mypy?logo=readthedocs)](https://mypy.readthedocs.io/en/stable/) 
- **pytest** The pytest framework makes it easy to write small tests, yet scales to support complex functional testing
  [![GitHub](https://img.shields.io/badge/GitHub-2B3137?logo=github&logoColor=white)](https://github.com/pytest-dev/pytest)
  [![PyPI](https://img.shields.io/badge/-PyPI:%20pytest-3775A9?logo=pypi&logoColor=white)](https://pypi.org/project/pytest)
  [![Docs](https://img.shields.io/badge/Sphinx-0A507A?logo=sphinx)](https://docs.pytest.org/en/latest)
- **ruff** An extremely fast Python linter and code formatter, written in Rust.
  [![GitHub](https://img.shields.io/badge/GitHub-2B3137?logo=github&logoColor=white)](https://github.com/astral-sh/ruff)
  [![PyPI](https://img.shields.io/badge/-PyPI:%20ruff-3775A9?logo=pypi&logoColor=white)](https://pypi.org/project/ruff)
  [![Docs](https://img.shields.io/badge/MkDocs-526CFE?logo=materialformkdocs&logoColor=white)](https://docs.astral.sh/ruff) 

Use the following commands to setup your local environment with development dependencies:

```bash
pip install --user poetry
poetry install --with dev
```

## Acknowledgements

* [BitVM](https://github.com/BitVM/BitVM) by the amazing
  [@RobinLinus](https://github.com/RobinLinus) and [@ZeroSync](https://github.com/ZeroSync) team.

## License

This project is licensed under the terms of the MIT license.
