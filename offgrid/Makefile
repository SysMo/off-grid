clear:
	clear; echo Hello

run/data-source: clear
	cargo run --bin example-data-source

run/morse-home: clear
	cargo run --bin morse-home

ARCH=aarch64-unknown-linux-gnu
DEST=sysmo@sysmo-rpi-01:/home/sysmo/workspace/off-grid/test

build/rpi:
	clear; 
	cross build --release --bin morse-home --target ${ARCH}
	cross build --release --bin data-source --target ${ARCH}

deploy/rpi:
	scp target/${ARCH}/release/morse-home ${DEST}
	scp target/${ARCH}/release/data-source ${DEST}
	scp -r config/* ${DEST}/config
