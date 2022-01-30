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
