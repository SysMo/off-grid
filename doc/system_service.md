# Creating system service

https://pudding-entertainment.medium.com/deploying-a-rust-web-service-on-raspberry-pi-4-practical-guide-with-cloudflare-tunnels-d862211ce47

## Service file: project-name.service

```ini
[Unit]
Description=Project Name Backend
After=network.target

[Service]
User=myuser
Group=myuser
Restart=on-failure
ExecStart=/opt/backend/project-name
WorkingDirectory=/opt/backend
StandardOutput=append:/var/log/project-name/backend.log
StandardError=append:/var/log/project-name/backend.log

[Install]
WantedBy=multi-user.target
```

## Watcher

project-name-watcher.service

```ini
[Unit]
Description=Project Name restarter - to reload the service when something changes on disk
After=network.target
StartLimitIntervalSec=60
StartLimitBurst=20

[Service]
Type=oneshot
ExecStart=/usr/bin/systemctl restart project-name.service

[Install]
WantedBy=multi-user.target
```

project-name-watcher.path

```ini
[Path]
Unit=project-name-watcher.service
PathModified=/opt/backend

[Install]
WantedBy=multi-user.target
```

```bash
systemctl start project-name.service
systemctl start project-name-watcher.service
```