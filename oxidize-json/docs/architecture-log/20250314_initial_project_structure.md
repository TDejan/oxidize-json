Main goal is to set up some structure that will eventually help us to evolve the project with less headache.

- Single file with everything on it.
- Move part of your code to a different module. (this step is missing in the repo, I forgot, sue me), please don't sue me.
- Move the module to a different file.
- Turn the module in a single file into a folder acting as a module with multiple files as sub-modules.
- Split your crate in a library and an executable that lives in the same directory tree.
- Move the executable and library to different crates (workspaces) with their own directory tree.