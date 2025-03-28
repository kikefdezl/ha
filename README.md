# Home Automation

Automating stuff around my house.

- The main backend code for the server and automations is in `src/` and is written in Rust using Axum. 
- Some external services are located in `services/`. 
- The whole setup is orchestrated with docker-compose to make deployment a breeze.

---

# Instructions

**Requirements**: Must have `make`, `docker` and `docker-compose` on the host machine.

1. Set environment variables

```bash
cp .env.example .env
```
Then fill `.env` with the appropriate values.

2. Build & Run

```bash
make build
make start
```

# Services

## Linak Controller

API that allows me to control the height of my Ikea Idasen desk.

### Gotchas

Setting up this device was tricky because bluetooth in Docker is a nightmare:
- Make sure bluetooth.service (bluez) is not running in the host (`sudo systemctl stop bluetooth`)
- For `linak-controller` to work, the device must have been paired before (else the commands will just hang). You will have to pair the device at some point the host (using e.g.`bluetoothctl`). This will save it as a paired device, which you can then mount onto the docker container by mounting `/var/lib/bluetooth/:/var/lib/bluetooth/:rw` as a volume. If you don't mount this volume, the `linak-controller` will appear to run but the command will just hang!
- There are often breaking changes in the contracts pushed to `linak-controller`.

### TODOs:

- [ ] Pass the port as an envvar from `.env` to make sure that the backend and service are always in sync. Right now the port in the service is hardcoded to `9123` and I'm passing that value to the backend via `.env`.
