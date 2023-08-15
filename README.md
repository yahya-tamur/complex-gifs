see the html document deployed by github on:

https://yahya-tamur.github.io/complex-gifs/doc/a.html

I've had this idea for a while, here's an older version:

https://github.com/yahya-tamur/Visualising-Complex-Functions

My biggest problem with that one was that it was hard to see what the point was
with just a folder of images and some code. I decided to make this version
after realizing that I could put animated gifs in a tex document if I compiled
it to html with htlatex instead of to pdf. This also had the advantage of being
easier to share, especially when hosted on github pages.

This version also adds the stereographic projection, and the domain coloring
for introducing the idea more gradually and more visually. It's multi-threaded.
It contains the code for all the images in the document, and makes it easier
to add more with a nice api.

gif is an interesting file format for this purpose.

It's compressed losslessly, and this process does sometimes make patterns (like
clouds of dots) which lossy formats like mp4 can mess up.

It's pretty easy to work with -- a frame is just a flattened 2d array of colors
(which is losslessly compressed by the library). This also means the math
behind rendering the plane or the sphere is done manually, though it is pretty
simple.

Viewing the images is cross-platform. I was worried they might be a little hard
to view since they're pretty large, but it did work on a pretty cheap android
phone.

There is a separate compilation step, but this also means none of the math is
being done by the computer viewing the image. There's no way to add
interactivity like clicking and dragging the sphere.

## Project Structure

- `complex-gifs` rust crate for creating the images

  - `examples` contains code for creating the images in the document

    To run an example file, run:

    `cargo run -r --example <example file> -- <output directory>`

    For example:

    `cargo run -r --example z_loop -- .`

  - `src` contains library for creating the images

- `doc` contains compiled document and images. If you compile locally, it will
  contain build files as well.

- `tex` contains code for compiling the document.

  - `a.tex` is the main tex file.

  Check `Makefile` for commands for building.

  `make image` runs every example file in the rust crate with output directory
  `../doc/images`.

  `make doc` compiles the tex file.

  `make clean` deletes the build files and the images.

The easiest way to create your own images might be to make a new example file
by copying one of the existing ones and running with the command written above.

## Goals

- (is it done?) figure out the fonts
- complex powers. z^a, a^z, etc.

### Done:

- create gifs to display the complex functions (similar to older project)
- maybe add domain coloring + black to white images
- htlatex for compiling latex into html
- for the rust project, one library file + separate binary for each image
- make gif of sphere with stereographic projection
- rayon to parallelize
- short writeup, like 5 pages max.
- graph more things, especially with the sphere, fourier series, e^e^e^z, ...
- Go over the document. How much do you expect a reader to know?
