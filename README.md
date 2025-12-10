# lib-juno
The backend library for both clients and servers using lib-juno. Look in examples/ for more.

to run the example server, run the following:
```bash
cd ./examples/simple_server/
cargo run -r -- -k ./keys/key.pem -c ./keys/cert.pem -m 1 --verbose
```
**You should never use the key and cert from the example server for anything other than testing, as they are publicly hosted on this project's github page**
To generate your own keys with `openssl`, you can refer to the TLS segment of [https://github.com/echo-narcissus/echo-prototype-v1/blob/main/Makefile]

