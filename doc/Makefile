doc: a.tex
	make4ht a.tex "mathjax"

image:
	cd ../complex-gifs && for ex in examples/*.rs; do cargo run --example "$$(basename "$${ex%.rs}")"; done;

clean: a.tex
	make4ht -m clean a.tex -a info "mathjax"

fmt: custom.css
	prettier -w custom.css

