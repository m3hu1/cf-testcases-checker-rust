# cf-testcases-checker-rust

<img src="https://rustacean.net/assets/rustacean-orig-noshadow.svg" alt="Rustacean" width="200" height="200">

A simple command-line tool written in Rust to automatically fetch and run test cases for Codeforces problems against your Golang solutions.

## Features

- Automatically fetches test cases from Codeforces problem pages
- Runs your Go solution against all test cases
- Provides colorized output showing test results
- Supports both contest and problemset URLs

## Usage

1. Clone the repository
```bash
git clone https://github.com/m3hu1/cf-testcases-checker-rust
```

```bash
cd cf-testcases-checker-rust
```

2. Build the project
```bash
cargo build --release
```

3. To globally install the binary, copy the binary to your `/usr/local/bin` directory
```bash
sudo cp target/release/cf-testcases-checker-rust /usr/local/bin
```

You can also run the binary directly from the project directory
```
cargo run [PROBLEM_ID]
```
> Example: `cargo run 4A`

4. Use the binary to run test cases against your Go solution
```bash
cfgo [PROBLEM_ID]
```
> Example: `cfgo 4A`

> [!NOTE]
> By default, the tool looks for Go solutions in `~/Documents/code/golang/{problem_id}/`. You can modify this path in the code or use the `--code-dir` flag.

## File Structure
Your Go solutions should be structured as follows:
```
~/Documents/code/golang/
├── 1847A/
│   └── main.go
├── 1847B/
│   └── main.go
...
```

## Dependencies
- Rust
- Go

## Note
This tool is specifically designed for testing Golang solutions for Codeforces problems.
