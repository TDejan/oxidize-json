Cargo.toml and Cargo.lock are stored in the root of your package (package root).
Source code goes in the src directory.
The default executable file is src/main.rs

Summary for this part:
Packages help you organize your codebase into manageable crates.
Crates are collections of modules, and they can be binary (executables) or libraries (reusable code). (oxidize-json is a crate as it contains an executable and it's a unit of compilation)
Modules organize code inside crates, and use brings items into scope. 
Paths provide a way to name and access items, either absolutely or relatively.
Privacy and scope control the accessibility of code, using pub for public items and default privacy for internal ones.