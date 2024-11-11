# DEV=/dev/ttyACM0
DEV=/dev/ttyUSB0

upython/firmware/c3:
	esptool.py --chip esp32c3 --port ${DEV} erase_flash
	esptool.py --chip esp32c3 --port ${DEV} --baud 921600 write_flash -z 0x0 scratch/ESP32_GENERIC_C3-20241025-v1.24.0.bin

upython/firmware/esp32:
	esptool.py --chip esp32 --port ${DEV} erase_flash
	esptool.py --chip esp32 --port ${DEV} --baud 921600 write_flash -z 0x1000 scratch/ESP32_GENERIC-20241025-v1.24.0.bin


upython/connect:
	mpremote connect ${DEV}

upython/ls:
	mpremote fs ls

upython/clean:
	mpremote run smbed/tools/cleanup.py

upython/install_pkgs:
	mpremote connect ${DEV} mip install logging
	mpremote connect ${DEV} mip install onewire
	mpremote connect ${DEV} mip install ds18x20
	mpremote connect ${DEV} mip install 'github:brainelectronics/micropython-modbus'

	
upython/upload/certs:
	mpremote fs mkdir /cert
	mpremote fs cp /data/Workspace/Hynergy/hyboard/cert/out/ca/ca-sysmo.crt :/cert/ca.crt
	mpremote fs cp /data/Workspace/Hynergy/hyboard/cert/out/client/sysmo.crt :/cert/sysmo.crt
	mpremote fs cp /data/Workspace/Hynergy/hyboard/cert/out/client/sysmo.key :/cert/sysmo.key


upython/upload:
	mpremote fs cp -rf smbed :/
	mpremote fs cp main.py :/
	mpremote fs cp config.json :/

	mpremote reset
	mpremote repl

upython/install/fresh:
	make upython/install_pkgs
	make upython/upload/certs
	make upython/upload

upython/repl:
	mpremote repl

