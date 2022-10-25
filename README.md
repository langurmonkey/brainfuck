# Langur's Brainfuck interpreter

Very small [Brainfuck](http://brainfuck.org) interpreter written in Rust.

You can compile with:

```bash
cargo build
```

and run with:

```bash
cargo run
```

After compiling, the binary is in `target/debug` (normal compile), or `target/release` (compiled with `cargo build --release`).

You need to give it an input program directly or via a file. If non is given, the interpreter reads the program from the standard input.

```bash
# Pass it a program file
brainfuck program.b 

# Or the program directly
brainfuck "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."

# Or just use the standard input
brainfuck < program.b
```

A few sample programs are in `programs/`. These are taken from [brainfuck.org](brainfuck.org).
