from machine import UART

class Rs485RtuConfig:
  def __init__(self, tx: int, rx:int, baudrate = 9600):
    self.tx = tx
    self.rx = rx
    self.baudrate = baudrate

  @staticmethod
  def from_dict(data: dict[str, object]) -> 'Rs485RtuConfig':
    return Rs485RtuConfig(
      tx = data['tx'],
      rx = data['rx'],
      baudrate = data['baudrate'],
    )    


class Rs485Rtu:
  def __init__(self, config: Rs485RtuConfig):
    self.uart = UART(
      1, baudrate = config.baudrate, 
      tx = config.tx, rx = config.rx
    )

  