# Audio Mic Record

This is an example showing how one can record audio from
the microphone and later play it back or save it to a file.

Language: GDScript

Renderer: GLES 2

Installing:
To start with, install Godot.
# https://godotengine.org/

Then you need to install rust.
# https://rustup.rs/

Then you need to install clang so you can compile the rust code into a format that can be used in Godot.
# https://rust-lang.github.io/rust-bindgen/requirements.html

Then you need to compile the rust code. From the terminal run
# cargo build --release

Then you can open the project in Godot.
Open the godot editor, click the import button and point it to the project.godot file.
Then you can run the project with the run button in the top right.

Whenever you edit the rust code, compile it again.