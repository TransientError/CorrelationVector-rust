## What is this?
This is a rust implementation of https://github.com/microsoft/CorrelationVector.

It's currently not complete, but it has the most basic features.

I mainly wrote this in order to generate arbitrary correlation vectors for testing, which this certainly works for.

## cvgen instructions
You can install the binary using
```
cargo install --path cvgen
```

Then, you can generate a correlation vector by simply running
```
cvgen
```

## Missing features
1. Atomic version
2. spin operation
