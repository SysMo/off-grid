build:
	. ${HOME}/export-esp.sh	&& cargo build

flash:
	. ${HOME}/export-esp.sh	&& espflash flash --baud=921600 --monitor target/riscv32imc-esp-espidf/debug/test1

monitor:
	espflash monitor -p /dev/ttyUSB0

