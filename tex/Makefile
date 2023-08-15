# Compile the document
doc: a.tex
	cd ../doc; make4ht ../tex/a.tex "mathjax"

# checks if all the images have been used
check:
	python3 images_used.py

# Create the images
image:
	rm -f ../doc/images/*.gif && cd ../complex-gifs && for ex in examples/*.rs; do cargo run -r --example "$$(basename "$${ex%.rs}")" -- "../doc/images/"; done;

# Clean build files. This will also delete the images!
clean: a.tex
	cd ../doc; make4ht -m clean ../a.tex -a info "mathjax"

# Format. The only relevant supported file type is css.
fmt: custom.css
	prettier -w custom.css
