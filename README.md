# Home Automation

Automating stuff around my house.

# Instructions

Must have `make`, `docker` and `docker-compose`. Then just run:

```bash
make build
make start
```

## Linak Controller

Allows me to control the height of my Ikea Idasen desk.

### Gotchas

Setting up this device was tricky because bluetooth in Docker is a nightmare:
- Make sure bluetooth.service (bluez) is not running in the host (`sudo systemctl stop bluetooth`)
- For `linak-controller` to work, the device must have been paired before (else the commands will just hang). You will have to pair the device at some point the host (using e.g.`bluetoothctl`). This will save it as a paired device, which you can then mount onto the docker container by mounting `/var/lib/bluetooth/:/var/lib/bluetooth/:rw` as a volume. If you don't mount this volume, the `linak-controller` will appear to run but the command will just hang!
- There are often breaking changes in the contracts pushed to `linak-controller`.
