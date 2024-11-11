import time

class Event:
  def __init__(self, timestamp):
    self.timestamp = time.localtime() if timestamp is None else timestamp

class ServiceMessage(Event):
  def __init__(self, msg, timestamp = None):
    self.msg = msg
    super().__init__(timestamp)
