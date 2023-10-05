# ic_grpc

This project attempt to be a "hello world" canister that implements grpc (protocol buffers).
Right now the example works with JSON / RPC !

Once deployed, you can call the http_request in command line (replace b77ix-eeaaa-aaaaa-qaada-cai by the canister_id from your local env):
```
wget -O - --post-data='{"jsonrpc": "2.0", "method": "hello", "params": [], "id":1}' http://localhost:4943/?canisterId=b77ix-eeaaa-aaaaa-qaada-cai | jq .
```

The goal would be to be able to call the http method as in the hello world of tonic: 
```
grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "whatever"}' '[::1]:50051' helloworld.Greeter/SayHello
```

To finally be able to discuss the same way the bitcoind plugin discuss with the teosd.

If you want to try you can call grpc request on the teosd tower in command line, call from teos sources (notes it might require some changes in the path of the includes from .proto files):

```
grpcurl -import-path ./teos/proto/teos/v2 -proto tower_services.proto -cacert=~/.teos/ca.pem -cert=~/.teos/client.pem -key=~/.teos/client-key.pem 'localhost:8814' teos.v2.PrivateTowerServices/get_tower_info
```

The canister needs to be reachable the same way to be able to be registered to the bitcoind teos plugin.

## TODO

Implement grpc (protocol buffer) instead of JSON / RPC in the canister.

## Misc

To start teosd tower:

```
bitcoind -rpcuser=user -rpcpassword=whatever -testnet -prune -noconnect
lightningd --testnet --bitcoin-rpcuser=user --bitcoin-rpcpassword=whatever
teosd --btcrpcuser=user --btcrpcpassword=whatever --btcnetwork=testnet
```

Note that the teosd plugin shall be added to the bitcoind plugin, see teos documentation

## Links
teos watchtower: https://github.com/talaia-labs/rust-teos/tree/master
Watchtower spec (seem not up to date with current implementation): https://github.com/sr-gi/bolt13/blob/master/13-watchtowers.md 
general spec lightning: https://github.com/lightning/bolts
tonic: https://github.com/hyperium/tonic