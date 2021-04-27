curl --location --request POST 'http://localhost:9933' \
--header 'Content-Type: application/json' \
--data-raw '{
    "id": 153,
    "jsonrpc": "2.0",
    "method": "contracts_call",
    "params": [
        {
            "origin": "5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM",
            "dest": "5CBcUAAfXA3MD6hmpLyyFojHUD9c222YGm3gfG4iGWGJwpdp",
            "value": 0,
            "gasLimit": 4999999999999,
            "inputData": "0xa93f6430"
        }
    ]
}'