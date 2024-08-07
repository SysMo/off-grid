clear:
	clear; echo Hello

run/data-source: clear
	cargo run --bin example-data-source

run/morse-home: clear
	cargo run --bin morse-home

ARCH=aarch64-unknown-linux-gnu
DEST_HOST=sysmo@sysmo-rpi-01
DEST_PATH=/home/sysmo/service/morse-home

build/rpi:
	clear; 
	cross build --release --bin morse-home --target ${ARCH}
	# cross build --release --bin data-source --target ${ARCH}

deploy/rpi:
	ssh ${DEST_HOST} mkdir -p ${DEST_PATH}
	ssh ${DEST_HOST} mkdir -p ${DEST_PATH}/config
	scp target/${ARCH}/release/morse-home ${DEST_HOST}:${DEST_PATH}
	scp target/${ARCH}/release/data-source ${DEST_HOST}:${DEST_PATH}
	scp -r config/* ${DEST_HOST}:${DEST_PATH}/config
