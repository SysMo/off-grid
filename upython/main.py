from smbed.services import Runtime
import logging

logging.basicConfig(level=logging.DEBUG)
runtime = Runtime.from_config_file('config.json')
runtime.start()
