# MDEXEC

Extract code blocks from a Markdown file and pass them to a script
program in order to execute the code.

## Example

1. Build with `cargo build --release`.
2. Run `target/release/mdexec README.md bash` to execute the follwoing Bash script.

```bash
cksum src/main.rs
```

The output should be the following:
```
297368607 2950 src/main.rs
```
