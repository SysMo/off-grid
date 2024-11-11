import network
import ntptime
import utime
import machine
try:
    import logging
except:
    class Logger:
        def __init__(self, name: str):
            self.name = name
        def info(self, msg: str):
            print(msg)
        def warn(self, msg: str):
            print(msg)
        def debug(self, msg: str):
            print(msg)
        def error(self, msg: str):
            print(msg)
    class logging:
        @staticmethod
        def getLogger(name: str):
            return Logger(name)


class WiFiService:
    def __init__(self, networks):
        self.stored_networks = networks
        self.sta_if = network.WLAN(network.STA_IF)
        self.logger = logging.getLogger("WiFi")

    @property
    def connected(self) -> bool:
        return self.sta_if.isconnected()
    
    def start(self):
        if self.connected:
            self.logger.info("Already connected to WiFi")
        else:
            self.logger.info('Scanning for networks ...')
            self.sta_if.active(True)
            ssid_list = self.sta_if.scan()
            ssid_list = [network[0].decode('UTF-8') for network in ssid_list]
            self.logger.info(ssid_list)
            for ssid in ssid_list:
                if ssid in self.stored_networks:
                    self.try_connect(ssid, self.stored_networks[ssid])
                    if self.connected:
                        break

        if self.connected:
            self.logger.info(f'Network config: {self.sta_if.ifconfig()}')
            self.sync_time()

    def try_connect(self, ssid, password, timeout = 15):
        self.logger.info(f"Trying to connect to {ssid} ...")
        t_start = utime.time()
        self.sta_if.connect(ssid, password)
        while utime.time() - t_start < timeout:
            utime.sleep(1)
            if self.sta_if.isconnected():
                self.logger.info(f"Connected to {ssid}")
                return True
        self.logger.info(f"Failed to connect to {ssid}")
        return False

    def sync_time(self):
        rtc = machine.RTC()
        self.logger.info("Syncing time ...")
        ntptime.settime()
        self.logger.info(rtc.datetime())