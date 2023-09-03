Converts text in a file to different cases

Different Case Types in the Programming World
 - camelCase
 - snake_case
 - kebab-case
 - PascalCase
 - MACRO_CASE
 - Train-Case

Usage: text_case_converter --from <from_case_type> --to <to_case_type> --path <file>

Options:
  -f, --from <from_case_type>  Sets the type of case to convert from
  -t, --to <to_case_type>      Sets the type of case to convert to
  -p, --path <file>            Sets the input file to use
  -h, --help                   Print help
  -V, --version                Print version

Example:

```
/file1.js
function someFunction() {
    const someValue = 10;
    doSomething(someValue);
}
```

running
```
cargo run -- -f camel -t snake /file1.js
```

will create a new file

```
function some_function() {
    const some_value = 10;
    do_something(some_value);
}
```
