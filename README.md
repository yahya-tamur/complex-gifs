see the html document deployed by github on:

https://yahya-tamur.github.io/visualizing-complex-functions-2/doc/a.html

in progress. rewrite of older project I had.

# Running the Program Locally

The rust crate on 'complex-gifs' contains the library for making the images as
well as the code the create each image as examples.

From the 'doc' folder, 'make doc' compiles the tex into html (may require various packages).
'make image' runs every example of the cargo crate to make the necessary images.
'make clean' deletes the build files, which unfortunately includes the images created
by 'make image'.

# Goals

- create gifs to display the complex functions (similar to older project)
- make gif of sphere with stereographic projection
- rayon to parallelize?
- short writeup, like 5 pages max?

done:

- maybe add domain coloring + black to white images
- htlatex for compiling latex into html
- for the rust project, one library file + separate binary for each image
