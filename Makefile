

all: clean patch generate form fmt build

clean:
	rm -rf src/

patch:
	svd patch svd/patches/esp32.yaml
	mv svd/esp32.svd.patched svd/esp32.svd

generate:
	svd2rust --target none -i svd/esp32.svd

form:
	form -i lib.rs -o src/
	rm lib.rs

fmt:
	cargo fmt

build:
	cargo clean
	cargo build
