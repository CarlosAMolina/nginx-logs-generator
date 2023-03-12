## Introduction

This project generates Nginx logs. 

## Run

To know how to run the program, we can get help by typing:

```bash
$ cargo run
Usage
    cargo run String Vec<f32>
        The first argument is the path where the `log` folder will be created to save the log files.
        The next arguments are the size (Gigabyte) of each log file to be generated.
    Example:
        cargo run /tmp 1.5 0.5 1
```

All files after the third one will be compressed.

Example, executing the example command:

```bash
cargo run /tmp 1.5 0.5 1
```

These files will be created in `/tmp/logs`:

- access.log.2.gz, 110 MiB (1.4 GiB uncompressed).
- access.log.1, 477 MiB.
- access.log, 954 MiB.


## Test

```bash
cargo test
```

