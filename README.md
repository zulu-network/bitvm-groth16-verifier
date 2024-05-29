# bitvm-groth16-verifier

It's a script of groth16 verifier leverage thw power of [On Proving Pairings](https://eprint.iacr.org/2024/640).

You can find out the rust version [here](https://github.com/zulu-network/prove-on-pairing).



## How to run
```bash
cargo run --release -- -nocapture
```

```bash
nohup cargo run --release -- --nocapture > logs/log-1 2>&1 &
```




## Reference
* [On Proving Pairings](https://eprint.iacr.org/2024/640)
