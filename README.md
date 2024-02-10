# PyHaiku

## Description

PyHaiku is a command-line tool designed to streamline the process of switching Python versions within Poetry virtual environments on Windows systems. Leveraging the "py" utility provided by the Windows installation of Python, PyHaiku manages the installation paths of different Python versions.

When working with Poetry, users may encounter challenges when attempting to switch Python versions using poetry env use `<python-version>`. This is because Poetry lacks direct access to the Python versions managed by the "py" utility, often resulting in errors indicating the inability to locate the desired Python version. Additionally, Poetry's env use command can accept a path to a Python installation to create a virtual environment, adding complexity to the process.

PyHaiku addresses these issues by serving as a convenient solution for Python developers on Windows who utilize Poetry. It enables effortless switching between different Python versions within virtual environments. Without PyHaiku, users would need to manually retrieve the location of their desired Python version's installation using `py -0p` and then execute poetry env use `<path-to-python-install>`.

For developers frequently switching Python versions for tasks such as testing new features or ensuring version compatibility, the manual execution of these commands can become cumbersome. PyHaiku aims to simplify this process, providing a user-friendly and efficient way to manage Python versions within Poetry virtual environments on Windows systems.

Note this issue may be subverted if a developer is using more elaborate development environments such as docker containers

## Installation

This tool is only intended for use on windows machines using the "py" utility provided by the windows distribution of python.

Make sure you have the following dependencies installed:

- rustup/cargo
- Poetry
- py

### Steps

1. Build the executable using `cargo build --release`.
2. Copy the executable to your Python project directory (located in target/release).

## Usage

1) Open a terminal in your project folder where .venv is located
2) Ensure VSCode or any application that may be using the python interpreter located in the virtual environment is closed
3) run `pyhaiku.exe <python version>` in terminal

Example Usage: `pykaiku.exe 3.12`

## Roadmap (TODOS)

- Support for specific patches of python versions
- More concise and streamline installation process
- Written test
- Github actions
