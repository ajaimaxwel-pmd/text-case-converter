# Text Case Converter

## Overview
The Text Case Converter is a command-line tool designed to convert the casing style of text within files. This is particularly useful for programmers dealing with codebases that use multiple naming conventions. The tool supports a wide variety of case types, making it a versatile utility for all your text transformation needs.

## Supported Case Types
The following case types are supported:

- `camelCase`
- `snake_case`
- `kebab-case`
- `PascalCase`
- `MACRO_CASE`
- `Train-Case`

## Installation
To get started, clone the repository and build the tool using Cargo:

```bash
git clone https://github.com/ajaimaxwel-pmd/text-case-converter.git
cd text_case_converter
cargo build --release
cargo run -- --from <from_case_type> --to <to_case_type> --path <file_path>
```

## Usage

To convert the text in a file, use the following command:

```bash
text_case_converter --from <from_case_type> --to <to_case_type> --path <file_path>
```

### Options

- `-f, --from <from_case_type>`: Specifies the original casing style.
- `-t, --to <to_case_type>`: Specifies the target casing style.
- `-p, --path <file_path>`: Specifies the path of the file to be converted.
- `-h, --help`: Displays help information.
- `-V, --version`: Displays the current version of the tool.

## Example

Consider a JavaScript file (`file1.js`) with the following content:

```javascript
function someFunction() {
    const someValue = 10;
    doSomething(someValue);
}
```

To convert all camel-cased identifiers to snake_case, run the following command:

```bash
cargo run -- -f camel -t snake /file1.js
```

This will generate a new file with the following transformed content:

```javascript
function some_function() {
    const some_value = 10;
    do_something(some_value);
}
```

## Contributions

We welcome contributions! If you encounter any issues or have feature requests, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
