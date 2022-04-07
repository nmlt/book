## Closures: Anonymous Functions that Can Capture Their Environment

Rust’s closures are anonymous functions you can save in a variable or pass as
arguments to other functions. You can create the closure in one place and then
call the closure to evaluate it in a different context. Unlike functions,
closures can capture values from the scope in which they’re defined. We’ll
demonstrate how these closure features allow for code reuse and behavior
customization.

### Capturing the Environment with Closures

The first aspect of closures we're going to examine is that closures can
capture aspects of the environment they're defined in for later use. Here's the
scenario: A t-shirt company gives away a free shirt to someone on their mailing
list every so often. People on the mailing list can optionally add their
favorite color to their profile. If the person chosen to get the free shirt has
their favorite color in their profile, they get that color shirt. If the person
hasn't specified a favorite color, they get the color that the company
currently has the most of.

There are many ways to implement this. For this example, we're going to use an
enum called `ShirtColor` that has the variants `Red` and `Blue`. The
company's inventory is represented by an `Inventory` struct that has a field
named `shirts` that contains a `Vec<ShirtColor>` representing the shirts
currently in stock. The method `shirt_giveaway` defined on `Inventory` gets the
optional shirt color preference of the person getting the free shirt, and
returns the shirt color the person will get. This setup is shown in Listing
13-1:

<span class="filename">Filename: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

<span class="caption">Listing 13-1: Framework of the shirt company giveaway
situation</span>

The `store` defined in `main` has two blue shirts and one red shirt in stock.
Then it calls the `giveaway` method for a user with a preference for a red
shirt and a user without any preference. Running this code prints:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Again, this code could be implemented in many ways, but this way uses concepts
you've already learned, except for the body of the `giveaway` method that uses
a closure. The `giveaway` method takes the user preference `Option<ShirtColor>`
and calls `unwrap_or_else` on it. The [`unwrap_or_else` method on
`Option<T>`][unwrap-or-else]<!-- ignore --> is defined by the standard library.
It takes one argument: a closure without any arguments that returns a value `T`
(the same type stored in the `Some` variant of the `Option<T>`, in this case, a
`ShirtColor`). If the `Option<T>` is the `Some` variant, `unwrap_or_else`
returns the value from within the `Some`. If the `Option<T>` is the `None`
variant, `unwrap_or_else` calls the closure and returns the value returned by
the closure.

This is interesting because we've passed a closure that calls
`self.most_stocked()` on the current `Inventory` instance. The standard library
didn't need to know anything about the `Inventory` or `ShirtColor` types we
defined, or the logic we want to use in this scenario. The closure captured an
immutable reference to the `self` `Inventory` instance and passed it with the
code we specified to the `unwrap_or_else` method. Functions are not able to
capture their environment in this way.

### Creating an Abstraction of Behavior with Closures

#### Refactoring Using Functions

#### Refactoring with Closures to Store Code

### Closure Type Inference and Annotation

There are more differences between functions and closures. Closures don’t
usually require you to annotate the types of the parameters or the return value
like `fn` functions do. Type annotations are required on functions because
they’re part of an explicit interface exposed to your users. Defining this
interface rigidly is important for ensuring that everyone agrees on what types
of values a function uses and returns. But closures aren’t used in an exposed
interface like this: they’re stored in variables and used without naming them
and exposing them to users of our library.

Closures are typically short and relevant only within a narrow context rather
than in any arbitrary scenario. Within these limited contexts, the compiler can
infer the types of the parameters and the return type, similar to how it’s able
to infer the types of most variables (there are rare cases where the compiler
needs closure type annotations too).

As with variables, we can add type annotations if we want to increase
explicitness and clarity at the cost of being more verbose than is strictly
necessary. Annotating the types for the closure we defined in Listing 13-5
would look like the definition shown in Listing 13-7.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs:here}}
```

<span class="caption">Listing 13-7: Adding optional type annotations of the
parameter and return value types in the closure</span>

With type annotations added, the syntax of closures looks more similar to the
syntax of functions. The following is a vertical comparison of the syntax for
the definition of a function that adds 1 to its parameter and a closure that
has the same behavior. We’ve added some spaces to line up the relevant parts.
This illustrates how closure syntax is similar to function syntax except for
the use of pipes and the amount of syntax that is optional:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

The first line shows a function definition, and the second line shows a fully
annotated closure definition. The third line removes the type annotations from
the closure definition, and the fourth line removes the brackets, which are
optional because the closure body has only one expression. These are all valid
definitions that will produce the same behavior when they’re called. Calling
the closures is required for `add_one_v3` and `add_one_v4` to be able to
compile because the types will be inferred from their usage.

Closure definitions will have one concrete type inferred for each of their
parameters and for their return value. For instance, Listing 13-8 shows the
definition of a short closure that just returns the value it receives as a
parameter. This closure isn’t very useful except for the purposes of this
example. Note that we haven’t added any type annotations to the definition: if
we then try to call the closure twice, using a `String` as an argument the
first time and a `u32` the second time, we’ll get an error.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs:here}}
```

<span class="caption">Listing 13-8: Attempting to call a closure whose types
are inferred with two different types</span>

The compiler gives us this error:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

The first time we call `example_closure` with the `String` value, the compiler
infers the type of `x` and the return type of the closure to be `String`. Those
types are then locked into the closure in `example_closure`, and we get a type
error if we try to use a different type with the same closure.

### Storing Closures Using Generic Parameters and the `Fn` Traits


[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else