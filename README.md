# Nex

> A strict, modern programming language with explicit effects.

**Status:** `v0.0.1` — compiler in development

**Note:** Some examples in this README describe planned or experimental language features and may not be supported by the current compiler.

Nex is a programming language focused on making program behavior explicit and analyzable.

Two ideas are central to its design:

* **Strict value semantics** — Nex distinguishes constructs that produce values from those that do not.

* **Explicit effects** — Nex tracks what a callable is capable of doing through effect checking.

Nex is experimental and under active development. Its syntax, semantics, compiler architecture, and features may change as development continues.

## Hello world

A minimal Nex program declares its callable interface separately from its implementation:

```nex
proc main effects StdOut;

def main {
    display("Hello world!");
}
```

The `proc` declaration specifies that `main` is a procedure and that it may perform the `StdOut` effect. The `def` declaration provides its implementation.

When no parameter type is specified, it is implicitly `()`, the unit type. Thus, `proc main effects StdOut;` is equivalent to `proc main : () effects StdOut;`.

A **statement** does not itself produce a value. Examples of statements include:

* **Control-flow**

  * `if`
  * `switch`

* **Control-flow terminators**

  * `return`
  * `leave`
  * `break`

* **Calls**

  * procedure calls

An **expression** produces a value and can therefore participate in value-producing contexts. Examples of expressions include:

* **Control-flow**

  * `when`
  * `match`

* **Calls**

  * function calls

For example, an `if` statement performs control flow but does not produce a value:

```nex
if condition {
    println("condition was true");
}
```

In contrast, `when` is an expression and produces a value:

```nex
let result = when condition {
    leave 10;
} else {
    leave 20;
};
```

The distinction is semantic rather than contextual. A construct does not become an expression merely because it appears where a value is expected. Whether a construct produces a value is determined by its semantics.

This distinction also applies to calls. A **procedure call is a statement**, while a **function call is an expression**.

A function can use an expression to determine the value returned by `return`:

```nex
func factorial : Int -> Int;

def factorial(n) {
    return when n <= 1 {
        leave 1;
    } else {
        leave n * factorial(n - 1);
    };
}
```

Here, `when` produces the value returned by `return`. The `leave` statements exit their respective blocks with a value; they do not return from `factorial` itself.

## Effects

Nex tracks not only **what a callable returns**, but also **what it is capable of doing**.

Effects are declared as part of a callable's interface and checked by the compiler:

```nex
func inspect : <T impl ToString> -> T effects StdOut;

def inspect(x) {
    println(x);
    return x;
}
```

The `effects StdOut` declaration indicates that `inspect` may perform the `StdOut` effect. A caller must therefore permit that effect when calling `inspect`.

Effects are separate from return values. A callable can return a value while performing effects, or return a value without performing any effects.

Because effects are part of a callable's interface, the compiler can reason about the behavior of calls without relying solely on their implementations.

### Divergence as an effect

Nex also tracks divergence as an effect through `MayDiverge`:

```nex
proc spin;
proc spin_diverge effects MayDiverge;

def spin {
    while true {}
}

def spin_diverge {
    while true {}
}
```

Both procedures have the same implementation, but only `spin_diverge` declares the `MayDiverge` effect.

A callable without `MayDiverge` is treated as terminating. `MayDiverge` indicates that a callable may not ever halt.

Divergence can affect compiler analysis and optimization. For example:

```nex
proc main effects StdOut;

def main {
    spin();

    println("Prints");

    spin_diverge();

    println("Doesn't print");
}
```

Here, `spin()` can be removed by the optimizer because its behavior has no observable effects in this context. `spin_diverge()` must be preserved because its possible divergence affects whether execution can reach the following statement.

## Systems programming

Nex is currently exploring systems programming as one of its primary targets.

The language aims to provide low-level control and predictable behavior while using explicit language semantics to make programs easier to reason about, analyze, and optimize.

This exploration includes problems commonly encountered in systems programming, including:

* value and non-value semantics
* explicit effect tracking
* strict callable interfaces
* compile-time analysis
* divergence
* generic constraints
* control-flow semantics

Nex is not yet committed to a final application domain. Its design is still evolving, and the direction of the language may change as its semantics and implementation develop.

## Design principles

Nex is being developed around a few core principles:

* **Strict semantics** — important distinctions are represented explicitly in the language rather than being hidden behind implicit conventions.

* **Explicit behavior** — values, return behavior, effects, and divergence are represented explicitly in callable interfaces and program semantics.

* **Compile-time reasoning** — Nex should provide the compiler with enough information to analyze program behavior and identify invalid or unnecessary operations.

* **Minimal implicit behavior** — language constructs should have clear, well-defined semantics rather than acquiring behavior from the context in which they appear.

* **Exploration over convention** — Nex does not aim to reproduce established language designs simply because they are familiar. It explores alternative solutions to problems in programming language design.

## Language features

Current development includes work around:

* typed functions and procedures
* explicit effects
* generic functions and constraints
* effect-aware compiler analysis
* divergence tracking
* compile-time optimization

Nex is still under active development, so this list should not be interpreted as a stable language specification.

## Examples

The repository contains small programs demonstrating Nex's syntax and language semantics, including:

* [`hello_world.nex`](examples/hello_world.nex) — basic program structure and output
* [`factorial.nex`](examples/factorial.nex) — functions, types, control flow, and return values
* [`inspect.nex`](examples/inspect.nex) — generics, constraints, effects, and return values
* [`spin.nex`](examples/spin.nex) — effects, divergence, and optimizer behavior

## Documentation

The [`docs`](docs) directory currently contains the formal grammar of Nex in [`grammar.ebnf`](docs/grammar.ebnf).

Nex uses custom grammar notation built around EBNF-style syntax with additional constructs for expressing grammar helpers and parser-specific rules. The development of this notation is separate from Nex and is not part of this repository. Documentation for the notation will be provided separately.

## Development status

Nex is under active development, and breaking changes may occur between releases.

Syntax, semantics, compiler internals, and language features are not yet considered stable.

## Roadmap

The following features are planned for the `v1.x` development line:

* LSP support
* default parameters
* string interpolation
* standard libraries
* macros
* richer diagnostics with notes, help, suggestions, and fix-its
* declaration and signature overloading
* archetypes
* return metadata
* intervals
* variable sets

These are development goals rather than guarantees. The roadmap may change completely as Nex evolves.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.