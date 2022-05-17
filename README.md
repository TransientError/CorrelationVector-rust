## What is this?
This is a rust implementation of https://github.com/microsoft/CorrelationVector.

I mainly wrote this in order to generate arbitrary correlation vectors for testing, which this certainly works for.

The repo consists of two parts:
### cvgen
cvgen is just a simple binary for creating a new correlation vector similar to uuidgen
#### cvgen instructions
You can install the binary using
```
cargo install --path cvgen
```

Then, you can generate a correlation vector by simply running
```
cvgen
```
### cvlib
cvlib aims to be an implementation of correlation vectors that you can use for rust code. 


### Missing features
1. (probably will not implement) v1 specification
