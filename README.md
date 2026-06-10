# MagniDraw

MagniDraw is a Rust graphics library. It provides a set of functions which enable the easy creation of 2D games. These functions are (currently): Rectangle drawing, sprite drawing, and text drawing.

MagniDraw is also a UI library, and can be used to create simple, themable, user interfaces. Right now, the builtin UI elements are: UI Rect, Label, Button, Separators, Margin containers, and Stacks. UI features can be enabled via the 'ui' feature.

This project wraps the much more generic [KeyDraw](https://github.com/carsonetb/KeyDraw) library to provide more easy-to-use features, although it is still possible to use it at a lower level.

Unlike `KeyDraw`, this project is **not** feature-complete, and the scope is currently unbound, although I do still intend to use it for future rendering projects in Rust.

## Examples?

Examples are located in the `examples` directory, they can be ran with `cargo run --example [name]`. Note that I haven't tested these examples on non-linux, non-wayland environments, so they might not work. These examples are very simple, and give a good representation of the current scope of the project.
