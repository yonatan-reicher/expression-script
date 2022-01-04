# Expression Script

My vision for a small scripting language that is pure and functional

## Installing

Run `git clone` on this repository as you would normally

## Running

Run with `cargo run [relative-path]`.

- `[relative-path]` is a file to execute and is optional.
- if `[relative-path]` is omitted, a REPL will start.

## Features

- Super basic types
- Functions and applications
- Partial application

## Syntax

### Types

- `any` - The most basic type.
- Function types - For example `any => any`. Right commutative.
  (`any => any => any` is `any => (any => any)`).

### Expressions

- Variable - Simply refer to a name. For example, `x`.
- Function - `x: any -> x`. An identifier, followed by an atom after a colon
with a body in the end. Right commutative.
- Type expression - `any => any`.
- Application - `(x: any -> x) x`.

