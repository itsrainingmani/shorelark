# Shorelark

A project to simulate evolution using neural networks and genetic algorithms in Rust

Source - [Learning to Fly](https://pwy.io/en/posts/learning-to-fly-pt1/)

## Why use a Workspace

Workspaces allow use to split a project into multiple, standalone crate with some advantages -

1. Easier to reason about a project's structure
2. Easy to setup and extend (crates can be kept in the same repo or multiple ones)
3. Allows for Cargo to compile the project's code in _parallel_

Workspaces allow you to introduce clear-cut boundaries between project modules!
