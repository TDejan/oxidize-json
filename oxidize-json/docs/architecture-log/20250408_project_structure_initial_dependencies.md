- goal is to create a project structure that would be able to scale well

- create oxidize project representing a cargo package
A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates. 

- organize project into crates
A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server.

- further modularize into tools, libs and services
Services will contain our user facing CLI tool for now, in the future some sort of a Web API will be introduced.

- since each lib represents a crate in itself, it will have its build defined in the Cargo.toml
Only have what's needed for that crate

- so each crate has it's own Cargo.toml, but there is only one Cargo.lock

- no circular dependencies allowed!
If core dependens on utils, then utils can't depend on core.
Only one dependency flow


oxidize/
├── Cargo.toml          # Root-level Cargo.toml for the workspace
├── src/
│   ├── main.rs         # Starts the CLI app
│   ├── lib.rs          # Core logic (optional at first)
│   └── cli/            # CLI arguments and command logic
│       ├── Cargo.toml  # Cargo.toml for the cli crate
│       ├── src/        # CLI-specific source folder
│       │   ├── commands/  # Commands folder inside src/cli
│       │   │   ├── mod.rs  # Module for commands (imports the specific commands)
│       │   │   └── read.rs      # "oxidize read ..." command logic
│       │   └── main.rs       # CLI args + command logic (definition of Cli struct)
│       │   └── cli.rs
├── README.md
└── .gitignore



Benefits
1. Separation of concerns
Each lib crate can encapsulate a specific domain or set of functionality (auth, core, etc.), making your codebase easier to reason about, maintain, and test.

2. Shared dependencies and faster builds
By using a single Cargo.lock and workspace-level caching, Rust avoids redundant builds and compiles dependencies only once for all crates. Huge time-saver.

3. Scalable architecture
Whether you're building a CLI, web server, or both, you can keep your business logic in libraries and compose them in different binaries (e.g., cli, web, etc.).

4. Reusability
You can potentially publish individual crates later or reuse them across other projects by versioning them locally or publishing to crates.io.

5. Better testability
Each crate can have its own test suite, and you can mock or stub behaviors more easily within a scoped context.

By using modules, we can group related definitions together and name why they’re related. Programmers using this code can navigate the code based on the groups rather than having to read through all the definitions, making it easier to find the definitions relevant to them. Programmers adding new functionality to this code would know where to place the code to keep the program organized.
<see cref="https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html"/>
