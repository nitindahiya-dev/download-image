<h2># Download Image</h2>

<p>This Rust project downloads an image from the web and saves it to a temporary directory.</p>

<p>## Features</p>

<li> Downloads an image from a specified URL.</li>
<li> Saves the image to a temporary directory.</li>
<li> Handles errors using the `error-chain` crate.</li>
<br>

<p>## Dependencies</p>

<p>This project uses the following dependencies:</p>

<li> [`tokio`](https://crates.io/crates/tokio): For asynchronous runtime.</li>
<li> [`reqwest`](https://crates.io/crates/reqwest): For making HTTP requests.</li>
<li> [`error-chain`](https://crates.io/crates/error-chain): For error handling.</li>
<li> [`tempfile`](https://crates.io/crates/tempfile): For creating temporary files and directories.</li>
<li> [`dirs`](https://crates.io/crates/dirs): For accessing system directories.</li>
<br>
<p>## Installation</p>

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone this repository:
    ```sh
    git clone https://github.com/yourusername/download-image.git
    ```
3. Navigate to the project directory:
    ```sh
    cd download-image
    ```
4. Build the project:
    ```sh
    cargo build
    ```

## Usage

Run the project with the following command:

```sh
cargo run
