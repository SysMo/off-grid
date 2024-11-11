import json
import machine
import asyncio
import time
import sys
import logging
import gc

from .mqtt import MqttService
from .wifi import WiFiService
from .dispatcher import Dispatcher
from ..hardware import LedIndicator
from .modbus import ModbusClientService
from ..data.register_maps.dts777 import DTS777
# from ..sensors import SensorManager
from ..util import CatchErrors, WithLogger, MqttLoggingHandler

# class RuntimeState:
#     Initilizing = 0
#     ConnectingWiFi = 1
#     SyncingTime = 2
#     ConnectingMQTT = 3

class Runtime(WithLogger):
    # state: RuntimeState
    # sensor_manager: SensorManager
    dispatcher: Dispatcher

    def __init__(self, config):
        super().__init__()
        self.logger.info("Starting Morse runtime")
        self.tick_interval = config["tick_interval"]

        # self.state = RuntimeState.Initilizing
        self.indicator_led = LedIndicator(config["led_indicator_pin"])
        self.indicator_led.on()

        self.dispatcher = Dispatcher()
        self.network = WiFiService(config["networks"])
        self.mqtt = MqttService.from_config(config["mqtt"])
        self.sensor_manager = SensorManager.from_config(config["sensors"], dispatcher = self.dispatcher)
        self.modbus = ModbusClientService(config["modbus"])


    @staticmethod
    def from_config_file(path: str):
        with open(path) as f:
            config = json.load(f)

        return Runtime(config)

    async def on_start(self):
        self.network.start()
        self.wdt = machine.WDT(timeout = 10 * self.tick_interval)
        self.mqtt.start()
        MqttLoggingHandler.start(self.mqtt)
        self.initialized = True

    async def on_tick(self):
        self.wdt.feed()
        self.logger.info(f'Ram: allocated: {gc.mem_alloc() / 1000 } kB, free {gc.mem_free() / 1000} kB')
        self.indicator_led.toggle()
        # self.mqtt.send_sensor_value("sensor1", 23.5)
        with CatchErrors(self.logger, "reading register snapshot"):
            register_snapshot = self.modbus.read_register_snapshot(DTS777.register_map())
            self.mqtt.send_register_snapshot(register_snapshot)

        # try:
        #     register_snapshot = self.modbus.read_register_snapshot(DTS777.register_map())
        #     self.mqtt.send_register_snapshot(register_snapshot)
        # except Exception as e:
        #     import io
        #     self.logger.error(f"Failed to read register snapshot:")
        #     sb = io.StringIO()
        #     sys.print_exception(e, sb)
        #     self.logger.error(sb.getvalue())

    #     await self.sensor_manager.acquire_data()
    #     await self.dispatcher.process_queue()

    async def main(self):
        await self.on_start()
        while True:
            await self.on_tick()
            await asyncio.sleep_ms(self.tick_interval)


    def start(self):
        asyncio.run(self.main())

    def soft_reset(self):
        sys.exit()

    def hard_reset(self):
        machine.reset()


# one_wire = sensor_manager.sensor_communicators["temperature"]

# while True:
#     one_wire.read()
#     for k, v in one_wire.values.items():    
#         services.mqtt.send_sensor_value(str(k), v)
#     time.sleep(1)


# async def main():
#     # while True:
#     #     one_wire.read()
#     #     for k, v in one_wire.values.items():    
#     #         runtime.mqtt.send_sensor_value(str(k), v)
#     #     print(one_wire.values)
#         await asyncio.sleep(1.)

# asyncio.run(main())