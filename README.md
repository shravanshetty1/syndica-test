# Syndica-test
Create a smart contract such that the program state can only be modified by the 2 authorized users. Most instruction validation has been skipped since this is only for demonstration purposes.

### Usage
Run a solana validator in a seperate terminal
```
solana-test-validator
```

Compile the solana program to bpf bytecode and deploy it to the solana cluster
```
cargo build-sbf
solana program deploy ./target/deploy/syndica_test.so
```


### Test
Replace the program id with the program id in your local machine
```
PROGRAM_ID=6jX27af1HDJjANdozTxNrwApdXzGVw3igjeYPdwXLagD cargo test -- --nocapture
```

