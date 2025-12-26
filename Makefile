help:
	cat Makefile

build:
	cargo build

run:
	cargo run

test:
	cargo test

clean:
	-rm output_*.png

compress-animation:
	for f in *.animation.png; do \
		apngasm --force -o "$$f" "$$f"; \
	done

compress-static:
	for f in *.static.png; do \
		pngquant --force --skip-if-larger --output "$$f" "$$f"; \
	done

compress: compress-static compress-animation
