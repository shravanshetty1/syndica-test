# Syndica-test
Create a smart contract such that the program state can only be modified by the 2 authorized users. Most instruction validation has been skipped since this is only for demonstration purposes.



### Test
You dont need to run a validator to run the tests
```
cargo test -- --nocapture
```


### Local setup
Run a solana validator in a seperate terminal
```
solana-test-validator
```

Compile the solana program to bpf bytecode and deploy it to the solana cluster
```
cargo build-sbf
solana program deploy ./target/deploy/syndica_test.so
```


