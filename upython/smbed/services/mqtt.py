from umqtt.simple import MQTTClient
import json
import time
import ssl
import logging
from ..protocols.modbus import RegisterSnapshot

class MqttStatus:
    Disconnected: int = 0
    Connected: int = 1

class MqttService:
    prefix: str
    def __init__(self, client_id: str, server: str, port: int, prefix: str, user: str = None, password: str = None):
        ssl_context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
        ssl_context.load_cert_chain(certfile="cert/sysmo.crt", keyfile="cert/sysmo.key")
        ssl_context.load_verify_locations(cafile = "cert/ca.crt")
        self.client = MQTTClient(
            client_id = client_id,
            server = server,
            port = port,
            ssl = ssl_context,

            # user = user,
            # password = password
        )
        self.prefix = prefix
        self.status = MqttStatus.Disconnected
        self.logger = logging.getLogger("Mqtt")

    @classmethod
    def from_config(cls, data):
        return MqttService(
            client_id = data["client_id"], 
            server = data["server"], 
            port = data["port"], 
            # user = data["user"], 
            # password = data["password"], 
            prefix = data["prefix"],
        )

    def start(self):
        self.logger.info("Connecting to MQTT broker ...")
        self.client.connect()
        self.logger.info("Connected to MQTT broker ...")
        self.status = MqttStatus.Connected

    def send_register_snapshot(self, register_snapshot: RegisterSnapshot):
        # topic = (self.prefix + sensor_id).encode()
        topic = self.prefix + "/data"
        payload = register_snapshot.serialize()
        msg = json.dumps(payload).encode()
        self.client.publish(topic, msg)
    
    def send_sensor_value(self, sensor_id: str, value: float):
        topic = (self.prefix + sensor_id).encode()
        payload = {'timestamp': int(time.time()) + 946677600 + 2 * 3600, 'value': value}
        msg = json.dumps(payload).encode()
        self.client.publish(topic, msg)

    def send_log_msg(self, msg: str):
        topic = (self.prefix + "/logs").encode()
        if self.status == MqttStatus.Connected:
            self.client.publish(topic, msg)

