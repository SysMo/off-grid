from ...data import Serializable, GlobalTime

class ModbusType:
  @classmethod
  def from_bytes(cls, bytes) -> 'ModbusType':
    raise NotImplementedError
  @classmethod
  def from_words(cls, words) -> 'ModbusType':
    raise NotImplementedError

class RegisterDef:
  offset: int
  def __init__(self, name: str, group: str, start: int, len: int, tpe: type[ModbusType]):
    self.name = name
    self.group = group
    self.start = start
    self.len = len
    self.tpe = tpe
  
class RegisterBlock:
  def __init__(self, name: str, start: int, len: int):
    self.name = name
    self.start = start
    self.len = len
    self.registers: list[RegisterDef] = list()

  def register(self, name: str, group: str, start: int, len: int, tpe: type[ModbusType]) -> 'RegisterBlock':
    register = RegisterDef(name, group, start, len, tpe)
    register.offset = register.start - self.start
    self.registers.append(register)
    return self

class RegisterMap:
  def __init__(self, blocks: list[RegisterBlock] | None = None):
    self.blocks: list[RegisterBlock] = list() if blocks is None else blocks



class RegisterValue(Serializable):
  def __init__(self, id: str, group: str, data: Serializable, tags: dict[str, str] | None = None):
    self.id = id
    self.group = group
    self.data = data
    self.tags = {} if tags is None else tags

  def serialize(self) -> object:
    return dict(
      id = self.id,
      group = self.group,
      data = {self.data.__class__.__name__: self.data.serialize()},
      tags = self.tags,
    )


class RegisterSnapshot(Serializable):
  def __init__(self, values: list[RegisterValue], timestamp: GlobalTime | None = None):
    self.timestamp = GlobalTime.now() if timestamp is None else timestamp
    self.values = values

  def serialize(self) -> object:
    return dict(
      timestamp = self.timestamp.serialize(),
      values = [value.serialize() for value in self.values]
    )
