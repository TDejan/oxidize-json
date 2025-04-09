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

