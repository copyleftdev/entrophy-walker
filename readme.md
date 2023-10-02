# Entropy Walker :sunglasses: :lock:

```
███████ ███    ██ ████████ ██████   ██████  ██████  ██   ██ ██    ██       ██     ██  █████  ██      ██   ██ ███████ ██████  
██      ████   ██    ██    ██   ██ ██    ██ ██   ██ ██   ██  ██  ██        ██     ██ ██   ██ ██      ██  ██  ██      ██   ██ 
██████  ██ ██  ██    ██    ██████  ██    ██ ██████  ███████   ████   █████ ██  █  ██ ███████ ██      █████   █████   ██████  
██      ██  ██ ██    ██    ██   ██ ██    ██ ██      ██   ██    ██          ██ ███ ██ ██   ██ ██      ██  ██  ██      ██   ██ 
███████ ██   ████    ██    ██   ██  ██████  ██      ██   ██    ██           ███ ███  ██   ██ ███████ ██   ██ ███████ ██   ██ 
```

## Created By 1337-SIGMA

---

## About :information_source:

Entropy Walker is a high-performance security tool that finds high entropy strings in files. High entropy strings often indicate sensitive information like passwords, API keys, and tokens.

---

## Features :star2:

- Fast and efficient, thanks to Rust's performance and the Rayon library for parallelism.
- Colorful and easy-to-read output.
- Customizable entropy threshold.

---

## Installation :wrench:

1. Clone the repository
    ```bash
    git clone https://github.com/1337-SIGMA/entropy-walker.git
    ```
2. Navigate to the project directory
    ```bash
    cd entropy-walker
    ```
3. Build the project
    ```bash
    cargo build --release
    ```

---

## Usage :computer:

Run the program with the following command:

```bash
./target/release/entropy-walker --directory /path/to/directory --entropy 3.0
```

- `--directory`: The directory to scan.
- `--entropy`: Optional. The entropy threshold. Default is 3.0.

---

## License :page_with_curl:

This project is licensed under the MIT License.
