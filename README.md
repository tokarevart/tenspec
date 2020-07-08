# tenspec

Polycrystalline tensile specimen generation and meshing software.

### Installation

1. Install [Gmsh](https://gmsh.info/) and [Neper](http://www.neper.info/).
   They must be available at the terminal as the command `gmsh` and `neper` at run time.
2. Install tenspec with `cargo install https://github.com/tokarevart/tenspec`.
   If you want to install Rust/Cargo, this is probably the easiest way: https://www.rustup.rs.

### Usage
First use `tess` module to generate initial tesselation,
then use `reg` module to regularize that tesselation
and only then mesh regularized tesselation with `mesh` module.

Every time you use a module its last result is cached
in `./tenspec-cache` directory, so, for example, you don't need
to generate and regularize tesselation multiple time to create meshes with
different characteristic lengths.

### Example

``` sh
$ tenspec tess -n 20 --dims 25 8 1 6 2 6  
$ tenspec reg --fmax 20 --sel 3 --mloop 5  
$ tenspec mesh --cl 3 -o tenspec-rough.msh
$ tenspec mesh --cl 1 -o tenspec.msh2
$ tenspec mesh --cl 0.3 -o tenspec-fine.key
```
