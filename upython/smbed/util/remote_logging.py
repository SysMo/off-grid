import logging
# from logging import Handler
# from ..services import MqttService

class MqttLoggingHandler(logging.Handler):
  def __init__(self, mqtt_service: object):
    self.mqtt_service = mqtt_service

  def emit(self, record):
    msg = self.formatter.format(record)
    self.mqtt_service.send_log_msg(msg)

  @staticmethod
  def start(mqtt_service: object):
    root_logger = logging.getLogger()
    formatter = logging.Formatter("%(levelname)s:%(name)s:%(message)s")
    mqtt = MqttLoggingHandler(mqtt_service)
    mqtt.setFormatter(formatter)
    root_logger.addHandler(mqtt)