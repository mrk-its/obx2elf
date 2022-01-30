# obx2elf

tool for converting MADS relocatable obx files into llvm-mos obj files in ELF format

```
USAGE:
    obx2elf [OPTIONS] <IN_FILE> <OUT_FILE>

ARGS:
    <IN_FILE>
    <OUT_FILE>

OPTIONS:
    -a, --align <ALIGN>
    -h, --help                     Print help information
    -l, --log-level <LOG_LEVEL>    [default: error]
    -V, --version                  Print version information
```

## Running example

1. open project directory in vscode with `Remote - Container` extension
2. in vscode terminal:
   ```
   cargo install --path .
   cd example; make
   ```

## License

All source code (including code snippets) is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.
