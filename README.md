# Development Guide 

This guide provides resources for setting up a todo app in Rust. For detailed instructions, please refer to:

1. [Rust Hello World Server Tutorial](https://medium.com/learning-rust/hello-world-server-8ad299d36cf5)
2. [Full Stack Todo Rust Course](https://github.com/brooks-builds/full-stack-todo-rust-course/tree/main/backend/rust/axum)
3. [Guide on Implementing Clean Architecture](https://blog.devgenius.io/creating-an-api-with-rust-clean-architecture-axum-and-surrealdb-2a95b1b72e0f)
4. [DDD, Hexagonal, Onion, Clean, CQRS, … How I put it all together](https://herbertograca.com/2017/11/16/explicit-architecture-01-ddd-hexagonal-onion-clean-cqrs-how-i-put-it-all-together/#ports)

To streamline your development process, consider using [Cargo Watch](https://github.com/passcod/cargo-watch). This utility automatically runs specified commands when files change, which is especially useful for recompiling your code or running tests whenever you save a file.

Install `cargo watch` with the following command:

```bash
cargo install cargo-watch
```


Once installed, you can use `cargo watch` to automatically recompile your code and restart your server whenever you save a file:

```bash
cargo watch -x run
```


For standardized commit messages, we recommend using Commitizen. More information can be found in the official Commitizen Tools documentation:



[Official Commitizen Tools Guide](https://commitizen-tools.github.io/commitizen/)

# Database

```bash
sea-orm-cli generate entity -o src/database/
```