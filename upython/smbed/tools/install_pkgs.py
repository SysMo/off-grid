# import network
# logging.basicConfig(level = logging.INFO)
# # station = network.WLAN(network.STA_IF)
# # station.connect('nasko-deco', 'rokobaroko')
# # # wait some time to establish the connection
# # station.isconnected()

# wlan = network.WLAN(network.STA_IF) # create station interface
# wlan.active(True)       # activate the interface
# wlan.scan()             # scan for access points
# logging.info(wlan.isconnected())      # check if the station is connected to an AP
# wlan.connect('nasko-deco', 'rokobaroko') # connect to an AP
# logging.info(wlan.config('mac'))      # get the interface's MAC address
# logging.info(wlan.ipconfig('addr4'))  # get the interface's IPv4 addresses
# print("Done")

import mip
import json
from ..services.wifi import WiFiService

def run(path: str = 'config.json'):
  with open(path) as f:
    config = json.load(f)
  wifi = WiFiService(config['networks'])
  wifi.start()
  installer = Installer()
  
  installer.logging()
  installer.modbus()


class Installer:
  def logging(self):
    try:
      import logging
    except ImportError:
      mip.install('logging')
      import logging
      logging.basicConfig(level = logging.INFO)
      logging.info("Package logging installed")

  def modbus(self):
    mip.install('github:brainelectronics/micropython-modbus')

if __name__ == '__main__':
  run()


