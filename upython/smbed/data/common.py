import time
import json

class Serializable:
  def serialize(self) -> object:
    raise NotImplementedError
  
  def to_json(self) -> str:
    return json.dumps(self.serialize())


class GlobalTime(Serializable):
  def __init__(self, value) -> None:
    self.value = value

  def serialize(self) -> object:
    value = self.value
    return f"{value[0]:02d}-{value[1]:02d}-{value[2]:02d}T{value[3]:02d}:{value[4]:02d}:{value[5]:02d}Z"
  
  @staticmethod
  def now() -> 'GlobalTime':
    return GlobalTime(time.gmtime())

