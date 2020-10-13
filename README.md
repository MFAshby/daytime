daytime
=======

An implementation of the [daytime](https://tools.ietf.org/html/rfc867) protocol.

Built in rust, using chrono and the async-std libraries, it should be able to 
handle quite a few requests.  

Tried with Apache JMeter testing the TCP server only, and locally on my pinebook 
it can process at least 16k requests / second, the limiting factor is probably _not_ 
the server but actually the test framework. 

Build and load test like:
```bash
cargo build --release 
target/release/daytime 

# In another terminal
jmeter -n -t "TCP Sampler.jmx" -l results.txt -e 
```
