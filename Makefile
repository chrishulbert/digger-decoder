help:
	cat Makefile

build:
	cargo build

run:
	cargo run

clean:
	-rm output_*.png

compress:
	for f in *.animation.png; do \
		apngasm --force -o "$$f" "$$f"; \
	done
