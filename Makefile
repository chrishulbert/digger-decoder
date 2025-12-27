help:
	cat Makefile

build:
	cargo build

run:
	cargo run data/lemmings
	
run-ohnomore:
	cargo run data/ohnomore

run-christmas91:
	cargo run data/christmas91

run-christmas92:
	cargo run data/christmas92

run-holidays93:
	cargo run data/holidays93

run-holidays94:
	cargo run data/holidays94

test:
	cargo test

clean:
	-rm output_*.png

compress-animation: *.animation.png
	for f in *.animation.png; do \
		apngasm --force -o "$$f" "$$f"; \
	done

compress-static: *.static.png
	for f in *.static.png; do \
		pngquant --force --skip-if-larger --output "$$f" "$$f"; \
	done

compress: compress-static compress-animation
