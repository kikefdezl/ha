#!/bin/bash

service dbus start
bluetoothd &

linak-controller --server --config config.yaml 
