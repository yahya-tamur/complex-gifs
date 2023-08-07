see the html document deployed by github on:

https://yahya-tamur.github.io/complex-gifs/doc/a.html

in progress. rewrite of older project I had.

## Running the Program Locally

The rust crate on `complex-gifs` contains the library for making the images
under the `src` directory, and the code for generating the images in the
document under the `examples` directory.

The `tex` folder contains the files that compile into the html document, as
well as scripts to generate all the images and compile the document.

The `doc` folder contains the compiled document as well as the generated
images.

## Goals

- graph more things, especially with the sphere, fourier series, e^e^e^z, ...
- Go over the document. How much do you expect a reader to know?
- figure out the fonts

Done:

- create gifs to display the complex functions (similar to older project)
- maybe add domain coloring + black to white images
- htlatex for compiling latex into html
- for the rust project, one library file + separate binary for each image
- make gif of sphere with stereographic projection
- rayon to parallelize?
