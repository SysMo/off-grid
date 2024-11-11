from machine import UART
from umodbus.serial import Serial as ModbusRTUMaster
from ..protocols.modbus import RegisterMap, RegisterSnapshot, RegisterValue
from ..util import WithLogger
# ModbusClientConfig = dict[str, object]
  

class ModbusClientService(WithLogger):
  def __init__(self, config: dict[str, object]):
    super().__init__()
    self.logger.info("Initializing Modbus")
    self.client = ModbusRTUMaster(
      pins = (config['tx'], config['rx']),
      uart_id = 1
    )
    self.slave_id = config['slave_id']
    self.logger.info("Done initializing Modbus")

  def read_register_snapshot(self, register_map: RegisterMap) -> RegisterSnapshot:
    self.logger.info("Requesting Modbus data")
    values: list[RegisterValue] = []
    
    for block in register_map.blocks:
      block_words = self.client.read_holding_registers(self.slave_id, block.start, block.len)
      for register_def in block.registers:
        register_words = block_words[register_def.offset:(register_def.offset + register_def.len)]
        register_data = register_def.tpe.from_words(register_words)
        values.append(RegisterValue(
          id = register_def.name, group = register_def.group,
          data = register_data,
        ))

    snapshot = RegisterSnapshot(values)
    # print(snapshot.serialize())

    return snapshot



