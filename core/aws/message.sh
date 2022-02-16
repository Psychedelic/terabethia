#!/bin/bash

curl -X POST https://zebdrmmi5b.execute-api.us-west-2.amazonaws.com/dev/hook -H "Content-Type: application/json" -d "{\"hash\": \"0x733b835f2b80301f5e4402569cdcff83d2b25b83c01cc9eedd5a8a900016f856\"}"
# curl -X POST http://localhost:3000/dev/message -H "Content-Type: application/json" -d "{\"hash\": \"0xf9fd3d48648f207858608685c27a714dbf7d7dfc83908d20b2914d710ebdcf90\"}"
