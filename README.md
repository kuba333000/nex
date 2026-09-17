# Nex

> A strict, modern systems programming language with explicit effects.

**Status:** `pre-v0.0.1` - Private

Nex is a systems programming language under active development. It is designed for systems-level programming while exploring modern solutions to language design rather than simply reproducing established patterns from older languages.

Two ideas are particularly central to Nex:

* **Strict value semantics** — Nex explicitly distinguishes constructs that produce a value from those that do not.
* **Explicit effects** — Nex tracks what a callable is capable of doing through effect checking.

The language is still experimental. Syntax, semantics, compiler architecture, and features may change before the first public release.

## Example

```nex
proc main effects StdOut;

def main {
    display("Hello world!");
}
```

## Statements and expressions

Nex makes a strict distinction between **statements** and **expressions**.

A statement performs an action but does not produce a value:

```nex
if condition {
    display("condition was true");
}
```

An expression produces a value and can therefore participate in value-producing contexts:

```nex
let result = when condition {
    leave 10;
} else {
    leave 20;
};
```

This distinction also applies to control-flow constructs such as `switch`/`match`, as well as calls.

Nex does not treat every construct that happens to appear in an expression-like position as implicitly producing a value. Whether something returns a value is part of its semantics.

## Return values

Nex also distinguishes between a callable that returns a value and one that does not.

Functions and procedures represent this distinction explicitly rather than treating the absence of a useful return value as merely another value.

For example:

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

This strict separation allows value-producing and non-value-producing code to remain distinct throughout the language.

## Effects

Nex is concerned not only with **what a callable returns**, but also with **what it is capable of doing**.

Effects are declared on callables and checked by the compiler:

```nex
func inspect : <T impl ToString> -> T effects StdOut;

def inspect(x) {
    display(x);
    return x;
}
```

A callable's effects form part of its interface. This allows the compiler to reason about operations beyond their return values.

Nex also tracks divergence as an effect:

```nex
proc spin;
proc spin_diverge effects MayDiverge;
```

This gives the compiler information about whether a callable may fail to return normally, which can participate in analysis and optimization.

## Systems programming

Nex is intended for systems programming, occupying the same broad problem space as languages such as C, C++, and Rust.

However, Nex is not intended to be a straightforward continuation of those languages. Its design explores different solutions to problems such as:

* value and non-value semantics
* explicit effect tracking
* strict callable interfaces
* compile-time analysis
* divergence
* generic constraints
* modern control-flow semantics

The goal is to provide systems-level control without assuming that established language designs are necessarily the only solutions.

## Design principles

Nex is being developed around several principles:

* **Strict semantics** — important distinctions are represented explicitly in the language.
* **Values are explicit** — expressions produce values; statements do not.
* **Explicit callable behavior** — return behavior and effects are part of callable interfaces.
* **Effect checking** — the compiler checks what operations a callable is permitted to perform.
* **Systems-level control** — Nex targets the kinds of programming traditionally associated with systems languages.
* **Independent design** — Nex explores new solutions instead of automatically inheriting conventions from existing languages.

## Language features

Current development includes work around:

* typed functions and procedures
* explicit effects
* generic functions and constraints
* expression-oriented control flow
* effect-aware compiler analysis
* divergence tracking
* compile-time optimization

Nex is still pre-release, so this list should not be interpreted as a stable language specification.

## Examples

The repository contains small programs demonstrating Nex's current syntax and semantics, including:

* `hello_world.nex` — basic program structure and output
* `factorial.nex` — functions, types, control flow, and return values
* `inspect.nex` — generics, constraints, effects, and returning values
* `spin.nex` — effects, divergence, and optimizer behavior

## Documentation

The `docs` directory currently contains the language grammar.

The grammar is intended as a reference for Nex's syntax; it does not by itself define all language semantics.

## Development status

Nex is currently private and under active development toward `v0.0.1`.

Expect breaking changes. Syntax, semantics, compiler internals, and language features are not yet considered stable.

## Roadmap

Longer-term goals include features such as:

* LSP support
* default parameters
* string interpolation
* standard libraries
* macros
* richer diagnostics with notes, help, suggestions, and fix-its
* declaration and signature overloading
* `for T` syntax
* archetypes
* return metadata
* intervals
* variable sets

These are development goals rather than guarantees for the first release.

## License

License information will be added as the project approaches its first release.
