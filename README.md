# 🚀 Full Stack Yew and Rocket Template

[![License](http://img.shields.io/badge/license-mit-blue.svg?style=flat-square)](LICENSE)
[![Server Build Status](https://github.com/wiseaidev/rocket-yew-starter-pack/workflows/server/badge.svg)](https://github.com/wiseaidev/rocket-rs/actions)
[![Client Build Status](https://github.com/wiseaidev/rocket-yew-starter-pack/workflows/client/badge.svg)](https://github.com/wiseaidev/rocket-rs/actions)

![Demo](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/nx4ttbcx91r0oi2tzc70.gif)

This full-stack template combines the power of [Yew](https://yew.rs/) on the frontend and [Rocket](https://rocket.rs/) on the backend to help you kickstart your web application development. It provides a solid foundation for building real-time, interactive, and responsive web applications.

## Features

- **Rust All The Way**: Write your web application entirely in Rust, ensuring type safety and performance.
- **Yew Frontend**: A modern, Rust-based frontend framework for building interactive web applications.
- **Rocket Backend**: A web framework for Rust with great flexibility and speed.
- **CRUD Operations**: Set up Create, Read, Update, and Delete operations easily.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- **Rust**: Make sure you have Rust and Cargo installed. If not, visit [rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started) for installation instructions.

- **Trunk**: This project uses Trunk for building the Yew frontend. You can install Trunk with Cargo by running:

    ```bash
    cargo install --locked trunk
    ```

    For more information about Trunk, visit [thedodd/trunk](https://github.com/thedodd/trunk).

- **wasm32-unknown-unknown Target**: To build WebAssembly files, you need to add the `wasm32-unknown-unknown` target to your Rust toolchain. You can add it by running:

    ```bash
    rustup target add wasm32-unknown-unknown
    ```

    This target is essential for compiling Rust code to WebAssembly.

## Getting Started

Follow these steps to get your project up and running:

1. **Use this template**: Click on the green "Use this template" Button.

1. **Clone the Repository**: Clone this repository to your local machine.

    ```bash
    git clone https://github.com/your-username/rocket-yew-starter-pack.git
    cd rocket-yew-starter-pack
    ```

1. **Install Dependencies**: Use `cargo` to install the required dependencies for both the frontend and the backend.

    ```bash
    # Install frontend dependencies
    cd ui
    trunk build

    # Install backend dependencies
    cd ../server
    cargo build
    ```

1. **Run the Application**: Start the backend server and the frontend development server.

    ```bash
    # Start the backend
    cargo run

    # Start the frontend development server
    cd ui
    trunk serve
    ```

1. **Access the Application**: Open your web browser and go to `http://localhost:8080` to access the application.

## Project Structure

The project follows a structured layout:

- `ui/`: Contains the Yew frontend code.
- `server/`: Contains the Rocket backend code.

## Usage

Here are some common tasks you can perform with this template:

- **Add API Routes**: Define your API routes in `src/main.rs`.
- **Modify Frontend**: Customize the frontend by editing the files in `ui/src/`.

## Contribution

Contributions are welcome! Feel free to submit issues, feature requests, or pull requests to improve this template.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Yew and Rocket communities for their amazing libraries and documentation.

Happy coding! 🚀🦀
