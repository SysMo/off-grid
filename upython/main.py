from smbed.services import Runtime
import logging

logging.basicConfig(level=logging.INFO)
runtime = Runtime.from_config_file('config.json')
runtime.start()
