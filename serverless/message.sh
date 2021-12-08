#!/bin/bash

# curl -X POST https://zebdrmmi5b.execute-api.us-west-2.amazonaws.com/dev/hook -H "Content-Type: application/json" -d "{\"hash\": \"0xf6150e78f9ff55007231f00b86deba1deac71ee328b5aeab14b631c25768fe80\"}"
curl -X POST http://localhost:3000/dev/message -H "Content-Type: application/json" -d "{\"hash\": \"0xf6150e78f9ff55007231f00b86deba1deac71ee328b5aeab14b631c25768fe80\"}"
