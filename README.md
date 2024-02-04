# MDEXEC - A tangle for Markdown

This program extracts code blocks from a Markdown file and passes them
to a script program in order to execute the code.  It implements a
tangle for Markdown.  See "[Literate Programming][1]" for further
explaination.

## Example

1. Build with `cargo build --release`.
2. Run `target/release/mdexec README.md bash` to execute the follwoing
   Bash script.

```bash
cksum src/main.rs
```

The output should be the following:
```
1647957730 2915 src/main.rs
```

## References

- Donald E. Knuth. "[Literate Programming][1]" (1984)

[1]: http://www.literateprogramming.com/knuthweb.pdf
