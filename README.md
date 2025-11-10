# Pantera
A Rust-based programming language designed to be human-readable. This is a personal project to learn the fundamentals of compiler engineering and as so, it is not a production-ready project.

## Features
- Custom IL compilation
- Basic data structures (array, objects, strings)
- Mark-and-sweep garbage collector
- Basic control flow statements (`if`, `loop`)
- intertwined function name with params (e.g. `fun compute(a)sum {...}`)
- Some basic std library functions

## Installation

1. `git clone https://github.com/radu2147/pantera`
2. `cd pantera/packages/pantera-cli`
3. `cargo build --release`
4. `./target/release/pantera ../../examples/even_nums.pant`

## Examples
Check the `examples/` folder or the ones below:

- Declaring functions
```rust
fun are_(a)_and_(b)_equal {
  return a's id is b's id;
}

print are_({id: 1})_and_({id:2})_equal;
```

- Iterating through arrays
```rust
var a = [1,2,3];
loop a {
  if it mod 2 is 0 {
    print it;
  }
}
```

- Input
```rust
var a = input();
print "Hello " + a;
```
