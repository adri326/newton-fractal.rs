# Newton's Fractal in Rust

This is a Rust implementation of the [Newton Fractal](https://en.wikipedia.org/wiki/Newton_fractal).

As of right now, all of the customization is done through hard-coded constants in `src/main.rs`.
The code is currently in a spaghetti state. Sorry for that :)

## Installation and running

You will need an installation of `rustc`+`cargo`. Then, `git clone` this repository, create the `output` directory and run with `--release`:

```sh
git clone https://github.com/adri326/newton-fractal.rs newton-fractal
cd newton-fractal
mkdir -p output

# You may edit src/main.rs now, to tweak some of the parameters, especially the resolution and the number of threads

cargo run --release
```
