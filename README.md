
# Simple File Server - Bounty Project

## Introduction
This project is a **Simple File Server** built with Rust, designed to serve directories and files over HTTP. It allows traversal of directories, supports file type recognition, and handles special characters (like CJK). The server also prevents backtracking beyond the root directory where it is launched.

## Prerequisites
To run this project, you'll need:
- [Rust](https://www.rust-lang.org/tools/install)
- A basic understanding of command-line interfaces

## Demo Video
<a href="https://youtu.be/XS6pT9TAR2w?feature=shared">
    <img src="https://github.com/SinghShubhamkumarkrishnadev/Bounty---Simple-File-Server/blob/main/preview.jpg" alt="Simple File Server Demo" width="400" height="300">
</a>


## Cloning the Repository
You can clone the project using the following command:
```
git clone https://github.com/SinghShubhamkumarkrishnadev/Bounty---Simple-File-Server.git
```

## Running the Project
After cloning the repository, follow these steps to run the server:
1. Navigate to the project directory:
    ```bash
    cd Bounty---Simple-File-Server
    ```
2. Build the project:
    ```bash
    cargo build
    ```
3. Run the project:
    ```bash
    cargo run
    ```
4. Open your browser and visit:
    ```
    http://localhost:5500
    ```

## Features
- **Serve Directories and Files**: Easily navigate directories and serve various file types.
- **File Type Detection**: Detect file types using the `infer` crate.
- **Special Character Handling**: Properly handle CJK and other special characters in URLs using `url-escape`.
- **Directory Traversal**: Navigate up and down directories, with the option to go back to the parent directory.
- **Backtracking Prevention**: Prevent users from accessing directories above the root.


