from ..services.dispatcher import Event

class SensorReadingFloat(Event):
  def __init__(self, sensor_id: str, value: float, timestamp = None):
    self.sensor_id = sensor_id
    self.value = value
    super().__init__(timestamp)

class SensorReadingError(Event):
  def __init__(self, sensor_id: str, msg: str, timestamp = None):
    self.sensor_id = sensor_id
    self.msg = msg
    super().__init__(timestamp)

class SensorInterfaceError(Event):
  def __init__(self, id: str, msg: str, timestamp = None):
    self.iface_id = id
    self.msg = msg
    super().__init__(timestamp)
