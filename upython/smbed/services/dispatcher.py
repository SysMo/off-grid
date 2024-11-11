from collections import deque

from .events import Event, ServiceMessage

class Dispatcher:  
  event_queue: deque[Event]

  def __init__(self) -> None:
    self.event_queue = deque((), 20, 1)

  def register_event_type(self, event_cls, handler):
    pass

  def dispatch(self, event: Event) -> None:
    try:
      self.event_queue.append(event)
    except IndexError as e:
      pass
  
  def format_timestamp(self, event: Event) -> str:
    (year, month, mday, hour, minute, second, _, _) = event.timestamp
    return f'{year}-{month}-{mday} {hour}:{minute}:{second}'

  async def process_queue(self):
    from ..sensors.events import SensorReadingFloat, SensorReadingError, SensorInterfaceError
    while len(self.event_queue) > 0:
      event = self.event_queue.popleft()
      if isinstance(event, ServiceMessage):
        print(f'{self.format_timestamp(event)}: {event.msg}')
      elif isinstance(event, SensorReadingFloat):
        print(f'{self.format_timestamp(event)}: Sensor {event.sensor_id}: {event.value}')
      elif isinstance(event, SensorReadingError):
        print(f'{self.format_timestamp(event)}: Error reading sensor {event.sensor_id}: {event.msg}')
      elif isinstance(event, SensorInterfaceError):
        print(f'{self.format_timestamp(event)}: Sensor interface error {event.iface_id}: {event.msg}')
      else:
        pass