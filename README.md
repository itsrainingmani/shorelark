# Shorelark

A project to simulate evolution using neural networks and genetic algorithms in Rust

Source - [Learning to Fly](https://pwy.io/en/posts/learning-to-fly-pt1/)

## Why use a Workspace

Workspaces allow use to split a project into multiple, standalone crate with some advantages -

1. Easier to reason about a project's structure
2. Easy to setup and extend (crates can be kept in the same repo or multiple ones)
3. Allows for Cargo to compile the project's code in _parallel_

Workspaces allow you to introduce clear-cut boundaries between project modules!

## Bindings

```rust
fn foo_1(items: &[f32]) {
    //   ^^^^^  ------
    //  binding  type
    // (immut.) (immut.)
}

fn foo_2(mut items: &[f32]) {
    //   ^^^^^^^^^  ------
    //    binding    type
    //   (mutable) (immut.)
}

fn foo_3(items: &mut [f32]) {
    //   ^^^^^  ----------
    //  binding    type
    // (immut.)  (mutable)
}

fn foo_4(mut items: &mut [f32]) {
    //   ^^^^^^^^^  ----------
    //    binding      type
    //   (mutable)   (mutable)
}

struct Person {
    name: String,
    eyeball_radius: usize,
}

fn decompose(Person { name, mut eyeball_radius }: Person) {
    //       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ------
    //                     binding                 type
    // (partially immutable, partially mutable) (immutable)
}
```

Bindings, contrary to types are local to a function. When the type is mutable, you can modify the thing being referenced. But if the binding remains immutable, you cannot modify _which_ thing is being referenced.

On the other hand, when the binding is mutable, you **can** change which thing is referenced. But if the type remains immutable, you cannot modify the thing itself.
