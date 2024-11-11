from .communicator import SensorCommunicator
from .one_wire import OneWireCommunicator
from ..services.dispatcher import Dispatcher

class SensorManager:
  sensor_communicators: dict[str, SensorCommunicator]

  def __init__(self, sensor_communicators: dict[str, SensorCommunicator], dispatcher: Dispatcher):
    self.sensor_communicators = sensor_communicators

  @staticmethod
  def from_config(data, dispatcher: Dispatcher) -> SensorManager:
    sensor_communicators = {}
    for sensor_id, sensor_definition in data.items():
      if isinstance(sensor_definition, dict) and len(list(sensor_definition)) == 1:
        for iface_type, iface_config in sensor_definition.items():
          if iface_type == "OneWire":
            try:
              sensor_communicators[sensor_id] = \
                OneWireCommunicator.from_config(iface_config, dispatcher)
              
            except ValueError as e:
              print(e)
          else:
            print(f"Unknown sensor type {iface_type}")
      else:
        print("Sensor interface definition should be a map with a single entry: {interface_type: interface_definition}")
        print(sensor_definition)
    return SensorManager(sensor_communicators, dispatcher)
  
  async def acquire_data(self):
    for id, comm in self.sensor_communicators.items():
      await comm.acquire_data()
