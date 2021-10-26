Hi,

Not a finished repo this is. Work in progress. 

The plan is to design wasm modules for basic tasks in the decentralized ecosystem, that can be pieced together ona computation network, in this case that is Fluence.  

While we see the emergence of protocols governed by DAO's, we can assume that these protocols will want to cooperate with services that are similar in nature. These would be services that can run without interference by a single human. Code updates - for example - can only be implemented by the governing entity. 

This could be an autnomous static site generator that publishes articles approved by am editorial DAO or a open data aggregation pipeline that is supported by multiple stakeholders. 

At the time of writing i have build proof of concepts for:  

0/ services that can be loaded from a remote ipfs node and removed after task has been completed. Imagine that you can get the cid of the wasm file from an ENS text record that can only be written by the governing entity. Hence, wasm code can be updated as a result of decentralised governance. 

1/ a data aggregation service that listens to events on the blockchain, fetches data froma subgraph, parses that data into an ipld structure and stores the root cid of that ipld tree as a text record on ENS.

2/ a script that runs every minute, creating and signing a transaction to update a text record on ENS. Relevent here is that private key used to ccreate the signature lives inside the memory of a fluence service (wasm). Stop the node and the private key is gone. 

With regard to /2 
  
Have a look at   
  
/aqua-scripts/interval.aqua  
/aqua-scripts/private.aqua  
/signer-service/facade/src  
/ens-service/facade  

You will notice that i "host" the compiled wasm files on a remote ipfs node. Also relevant, i use a ENS subdomain on rinkeby as a key value store. The cool thing is that you can set the controller of your subdomain with the controller of your parent domain. So, you ask the signer service to create a new address, of which ythe secret key remains inside the service, and then you set the controller of your ENS subdomain to the new address owned by the signer service. 

With regard to the aqua scripts: The compiled private.interval.air is "hosted" on ipfs. interval.start.air fetches that code and runs it with a minute interval. Have a look at package.json also. What i do to run the code is: 

a/ npm run deploy_signer  
b/ store new signer_service_id as ENS text record  
c/ ascribe ENS subdomain to new address  
d/ send funds to new address   
e/ npm run start_interval  

I want to write a proper blog post on this at a later date. Write me an email when you have remarks or ideas: joera@joermaulders.com 