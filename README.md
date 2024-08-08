# Flutter Test Code Generator

## Overview

In the development of a large Flutter application, integration testing often involves writing repetitive test code with varying parameters. This repetitive task consumes significant time and effort. Noticing that the pattern of code remained largely the same but with different parameters, I decided to build a tool using Rust to automate the generation of these repetitive code snippets. This tool streamlines the process, allowing developers to focus on more important tasks rather than manually writing similar code over and over.

## My Story

While working on a complex Flutter project, I encountered the challenge of writing integration test code that was essentially repetitive but varied in parameters. This repetitive task was time-consuming and tedious. I realized that the core pattern of the code remained the same, with only the parameters changing. This insight led me to build this Rust tool, designed to automate the generation of test code snippets. By using this tool, I could focus on more important tasks and improve the efficiency of my testing process. This solution not only sped up the testing phase but also ensured consistency in the test code, leading to a more streamlined development workflow.

## Features

- **Code Generation by Type**: Generates test code for various widget types such as `TextFormField`, `TextField`, `ElevatedButton`, and more.
- **Code Generation by Text**: Creates test code that interacts with widgets identified by text.
- **Code Generation by Key**: Produces test code for widgets identified by keys.
- **Code Generation by Icon**: Generates test code for widgets identified by icons.
- **Customizable Delays**: Includes options to insert delays and manage custom code snippets.

## Usage

### Running the Program

To use the Flutter Test Code Generator, compile and run the Rust program. The program will present a menu with options to generate code based on widget type, text, key, or icon.

### Menu Options

1. **By Type**: Choose from various widget types to generate test code.
2. **By Text**: Enter text to generate test code for widgets identified by that text.
3. **By Key**: Enter a key to generate test code for widgets identified by that key.
4. **By Icon**: Enter an icon name to generate test code for widgets identified by that icon.
5. **Exit**: Exit the program.

### Example

1. Run the program.
2. Select an option (e.g., "1. By Type").
3. Choose a widget type (e.g., `TextFormField`).
4. Enter additional parameters if required.
5. The generated code will be written to a file named `generator.txt` on your desktop.

## Code Explanation

Here's a brief overview of the code structure:

- **Imports**: The program uses standard libraries for input/output operations and file handling.

  ```rust
  use std::io;
  use std::fs::OpenOptions;
  use std::io::Write;
  use std::env;
  use std::path::{Path, PathBuf};
