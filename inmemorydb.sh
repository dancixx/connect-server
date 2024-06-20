#!/bin/sh

surreal start --log trace --user root --pass root --bind 127.0.0.1:8000 file:connect.db
# surreal start --log trace --user root --pass root --bind 127.0.0.1:8000 tikv://127.0.0.1:2379