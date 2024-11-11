import machine

class LedIndicator:
  def __init__(self, pin: int):
    self.pin = machine.Pin(pin, machine.Pin.OUT)
  
  def on(self):
    self.pin.on()

  def off(self):
    self.pin.off()

  def toggle(self):
    if self.pin.value() == 1:
      self.off()
    else:
      self.on()
