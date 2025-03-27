FROM python:3.11-slim

WORKDIR /app

RUN  apt-get update && \
	apt-get install -y bluez

RUN pip install linak-controller

COPY config.yaml config.yaml
COPY entrypoint.sh entrypoint.sh

ENTRYPOINT ["sh", "entrypoint.sh"]
