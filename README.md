# Home Automation

Automating stuff around my house.

---

# Instructions

**Requirements**: Must have `make`, `docker` and `docker-compose` on the host machine.

Then just run:
```bash
make start
```

Then you can access HA from `http://localhost:8123`.

**NOTE: If it's the first time** setting this up, you will have to create a new home/account, but then the configuration will be loaded properly after credential creation.

# Services

## Ikea Idasen

Control the height by running the `sit` and `stand` scripts. They can be triggered via API:

```bash
curl -X POST \
    -H "Authorization: Bearer $HOME_ASSISTANT_BEARER_TOKEN" \
    $HOME_ASSISTANT_URL/api/services/script/sit

curl -X POST \
    -H "Authorization: Bearer $HOME_ASSISTANT_BEARER_TOKEN" \
    $HOME_ASSISTANT_URL/api/services/script/stand
```
