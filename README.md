## Introduction

This project generates Nginx logs. 

## Run

The logs will be created in the `/tmp/logs` folder.

All files after the third one will be compressed.

Example, to create 3 log files of 1.5 GB, 0.5 GB and 1 GB:

```bash
cargo run 1.5 0.5 1
```

The previous command will create these files:

- access.log.2.gz, 110 MiB (1.4 GiB uncompressed).
- access.log.1, 477 MiB.
- access.log, 954 MiB.

You can get help with:

```bash
cargo run
```

## Test

```bash
cargo test
```

