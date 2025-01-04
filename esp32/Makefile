monitor:
	cargo espflash monitor

build:
	MCU=esp32c3 cargo build --release --target riscv32imc-esp-espidf

flash:
	MCU=esp32c3 cargo espflash flash --release  --target riscv32imc-esp-espidf --monitor --baud 921600


