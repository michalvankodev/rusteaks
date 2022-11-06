# Rusteaks ⚒

Rusteaks are tools to be 3D printed, designed with [OpenSCAD](https://openscad.org/), written in [Rust](https://www.rust-lang.org/)

## Generating own parts / Installation

To generate model parts you need to have rust installed. Follow [instructions on rust installation](https://www.rust-lang.org/learn/get-started) to get up and running.
In `main.rs` you can use the `create_scad` function to generate models. Just import the model that you want to print.

Generated `.scad` files can be opened with OpenSCAD software to be previewed and rendered into STL models.

For developing models, I recommend using [watch mode](https://github.com/watchexec/cargo-watch#readme):

```sh
cargo watch -x run
```

OpenSCAD will constantly use the latest version of the generated `.scad` file.

When you are done with designing the model. Proceed with the generation of the STL file. This STL file can be later on used with any slicer software for 3D printing. (e.g. [PrusaSlicer](https://www.prusa3d.com/))

## Parts

### Table attachment

Universal table attachment, that can be modified by parameters for any table height and personal preference.

### Headphone stand

Parametrized headphone stand that can be modified to personal preference. Can be put toghether with the [table attachment](#table-attachment).

