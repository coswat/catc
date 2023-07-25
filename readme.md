# Cat Clone

This program is a simple syntax highlighter written in Rust using the Syntect library. It reads the contents of a file and performs syntax highlighting based on the file's extension. The highlighted contents are then printed to the console.

## Usage

To use the syntax highlighter, follow these steps:

1. Make sure you have Rust installed on your system. If not, you can download and install it from the official Rust website: [https://rust-lang.org](https://www.rust-lang.org/)


2. Clone or download the source code of this program.

3. Open a terminal and navigate to the directory containing the source code.

4. Build the program by running the following command: 

```bash 
$ cargo build --release
$ mv target/release/catc /usr/local/bin/
```

5. After a successful build, you can run the program with the following command:

```bash
$ catc <filename>
```

Replace `<filename>` with the path to the file you want to highlight.

6. The program will read the contents of the file, perform syntax highlighting, and print the highlighted contents to the console.

## Examples

Here are a few examples of how to use the syntax highlighter:

- Highlight a HTML file:

```bash 
$ catc test.html
```

- Highlight a Rust file:

```bash 
$ catc src/main.rs
```

## Note

- If no file extension is provided or the file extension is not recognized, the program will default to using the "txt" extension for highlighting.

- The program uses the "base16-ocean.dark" theme for syntax highlighting. If you want to use a different theme, you can modify the `output` function in the code and replace `"base16-ocean.dark"` with the name of the desired theme.

That's it! You can now use this syntax highlighter to quickly visualize the syntax of various programming and markup languages. Feel free to modify and customize the program according to your needs.