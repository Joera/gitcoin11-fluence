

This is not an official tutorial. I got it working and i am sharing how i did it. I am not a good programmer and constructive criticism is most welcome. 

# ENS text records 


## Use cases 

Text records on an ENS domin name can be used as a key value store, allowing everyone to have read permissions and being sure only the private key of the controller ethereum account may write to it. It can therefore be used to validate cids as being correct by the owner. For example to point to the correct root of an ipfs dag tree that is being changed each time when underlying data are updated. Or to point at the most recent stable piece of code, stored on ipfs. The latter can be used to maintain updatable wasm services is to ask ens for the correct ipfs cid. 

## ingredients

The code can be found inside ./facade/src/ens_call.rs. ./build.sh compiles rust to wasm. 

There are two public methods: get_record and an update_record. The service makes use of the curl_adapter module to interact with the ens contract over the json rpc api and I am using an alchemyapi.io endpoint as ethereum provider. ENS allows us to use several testnets. I am using rinkeby with chain_id 4 here and i am hoping for an affordable layer 2 later. 

Ethers and web3 libraries do not fully compile to .wasm and cannot be used. The ethers-core crate does compile and helps with a range of useful types. The ethabi crate is used to load the abi for the resolver contract. An ENS domain name has a controller, which is the address that own its, and a resolver, which is the contract where the records are stored. You can build a custom resolver, but I am using the general one and copied the abi from etherscan. 

The resolver contract references an individual domain name by an "ens-node" id. You can calculate the node hash from a human-readable domain name here -> 
https://swolfeyes.github.io/ethereum-namehash-calculator/. IF you get a string from the online calculater, then convert that with a hex! macro from the hex-literal crate. The code now also contains a function to convert the domain name as a str to H256. 

## get records

We will use the eth_call method on the ethereum json rpc api to read a text record from the ens resolver contract. To make the call the curl request needs the method name (eth_call), the sender address, the resolver contract address, the hash of the input data, containing the ens_node id and the text record name, and finally the ethereum provider url. 

The ethabi interface helps us to create the input_data parameter for the curl call for the resolver contract requires the above described ens_node and the name of the record. 

ethers_core types are used to build the raw transation. ethabi types are used to format the inpiut data for the raw transaction, as well as the contract calls 

