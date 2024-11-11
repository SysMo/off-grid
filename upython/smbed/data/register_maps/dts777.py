from ...protocols.modbus import RegisterMap, RegisterBlock
from ..common import Serializable
from ...protocols.modbus import ModbusType

class DTS777:

  @classmethod
  def register_map(cls):
    return RegisterMap([
      RegisterBlock(name = "BL1", start = 0, len = 27)
        .register(name = "", group = "electrical", start = 0, len = 27, tpe = DTS777Electricity),
      RegisterBlock(name = "BL2", start = 29, len = 52)
        .register(name = "", group = "electrical", start = 29, len = 52, tpe = DTS777Energy),

      # RegisterBlock(name = "", group = "", start = , len = ))
    ])

class DTS777Electricity(ModbusType, Serializable):

  @classmethod
  def from_words(cls, words) -> ModbusType:
    return DTS777Electricity(words)

  def __init__(self, data: list[int]):
    self.data = data

  # v1-v3 voltages
  @property
  def v1(self) -> float:
    return self.data[0] / 10.0
  @property
  def v2(self) -> float:
    return self.data[1] / 10.0
  @property
  def v3(self) -> float:
    return self.data[2] / 10.0
  
  # i1-i3 currents
  @property
  def i1(self) -> float:
    return self.data[3] / 10.0
  @property
  def i2(self) -> float:
    return self.data[4] / 10.0
  @property
  def i3(self) -> float:
    return self.data[5] / 10.0

  # p1-p3 active phase powers
  @property
  def p1(self) -> float:
    return self.data[8]
  @property
  def p2(self) -> float:
    return self.data[9]
  @property
  def p3(self) -> float:
    return self.data[10]

  # q1-q3 reactive phase powers
  @property
  def q1(self) -> float:
    return self.data[12]
  @property
  def q2(self) -> float:
    return self.data[13]
  @property
  def q3(self) -> float:
    return self.data[14]

  # pf1 - pf3 power factors
  @property
  def pf1(self) -> float:
    return self.data[20] / 1000.0
  @property
  def pf2(self) -> float:
    return self.data[21] / 1000.0
  @property
  def pf3(self) -> float:
    return self.data[22] / 1000.0
  
  # frequency
  @property
  def f(self) -> float:
    return self.data[26] / 100.0

  def serialize(self) -> object:
    return {
      "v1": self.v1,
      "v2": self.v2,
      "v3": self.v3,
      "i1": self.i1,
      "i2": self.i2,
      "i3": self.i3,
      "p1": self.p1,
      "p2": self.p2,
      "p3": self.p3,
      "pf1": self.pf1,
      "pf2": self.pf2,
      "pf3": self.pf3,
      "f": self.f,
    }
  
  def __str__(self):
    return str(self.serialize)
  
class DTS777Energy(ModbusType, Serializable):
  def __init__(self, data: list[int]):
    # print(data)
    self.base_addr = 0x1d
    self.data = data

  @classmethod
  def from_words(cls, words) -> ModbusType:    
    return DTS777Energy(words)

  def get_value(self, i: int) -> float:
    ind = i - self.base_addr
    return 0.01 * ((self.data[ind] << 16) + self.data[ind + 1])

  @property
  def active(self) -> float:
    '''total active energy'''    
    return self.get_value(0x1d)
  @property
  def pos_active(self) -> float:
    '''positive active energy'''
    return self.get_value(0x27)
  @property
  def neg_active(self) -> float:
    '''reverse active energy'''
    return self.get_value(0x31)
  
  @property
  def reactive(self) -> float:
    '''total reactive energy'''
    return self.get_value(0x3b)
  @property
  def pos_reactive(self) -> float:
    '''positive reactive energy'''
    return self.get_value(0x45)
  @property
  def neg_reactive(self) -> float:
    '''reverse reactive energy'''
    return self.get_value(0x4f)

  def serialize(self) -> object:
    return dict(
      e_active = self.active,
      e_pos_active = self.pos_active,
      e_neg_active = self.neg_active,
      e_reactive = self.reactive,
      e_pos_reactive = self.pos_reactive,
      e_neg_reactive = self.neg_reactive,
    )