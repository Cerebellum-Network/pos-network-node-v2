curl --location --request POST 'http://localhost:9933' \
--header 'Content-Type: application/json' \
--data-raw '{"id":405,"jsonrpc":"2.0","method":"contracts_call","params":[{"origin":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","dest":"5CBcUAAfXA3MD6hmpLyyFojHUD9c222YGm3gfG4iGWGJwpdp","value":0,"gasLimit":4999999999999,"inputData":"0x81c336358eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"}]}'
