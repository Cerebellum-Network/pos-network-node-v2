curl --location --request POST 'http://localhost:9933' \
--header 'Content-Type: application/json' \
--data-raw '{
    "id": 4159,
    "jsonrpc": "2.0",
    "method": "contracts_call",
    "params": [
        {
            "origin": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            "dest": "5CBcUAAfXA3MD6hmpLyyFojHUD9c222YGm3gfG4iGWGJwpdp",
            "value": 0,
            "gasLimit": 4999999999999,
            "inputData": "0x106c8ecbd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
        }
    ]
}'