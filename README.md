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

## Type Parameters

Here we have a rust function definition

```rust
// visibility  generics   _ function parameters
// |          _|     ____|  (or just "parameters")
// |         |      |
// v-v       v-----vv----------v
   pub fn foo<'a, T>(bar: &'a T) { /* ... */ }
//            ^^  ^  ^--------^
//            |   |  |
//            |   |  function parameter
//            |   |  (or just "parameter")
//            |   type parameter
//            lifetime parameter
```

Reading this aloud, we get - "_public function `foo` is generic over a lifetime `a` and type `T` and accepts a single parameter named `bar` that is a reference to `T`_"

Now we look at the call site of the function

```rust
// v-----------------------v call site
   foo::<'static, f32>(&1.0);
//       ^-----^  ^-^  ^--^
//       |        |    |
//       |        |    function argument
//       |        |    (or just "argument")
//       |        type argument
//       lifetime argument
```

## The Night Train

In the `IntoIterator` implementation for Chromosome, we have a line that looks something like this -

```rust
impl IntoIterator for Chromosome {
    /*  */

    type IntoIter = impl Iterator<Item = f32>;

    /*  */
}
```

In order to enable this, we use a `nightly` feature called `min_type_alias_impl_trait` which allows us to use `impl Trait` in places such as associated types.

Otherwise, we would have to figure out the type on our own

```rust
impl IntoIterator for Chromosome {
    /*  */

    type IntoIter = std::vec::IntoIter<f32>;

    /*  */
}
```
