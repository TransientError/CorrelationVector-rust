## CorrelationVector for Rust
CorrelationVector-Rust provides the Rust implementation of the CorrelationVector protocol for tracing and correlation of events through a distributed system.

## CorrelationVector
### Background
Correlation Vector (a.k.a. cV) is a format and protocol standard for tracing and correlation of events through a distributed system based on a light weight vector clock. The standard is widely used internally at Microsoft for first party applications and services and supported across multiple logging libraries and platforms (Services, Clients - Native, Managed, Js, iOS, Android etc). The standard powers a variety of different data processing needs ranging from distributed tracing & debugging to system and business intelligence, in various business organizations.

For more on the correlation vector specification and the scenarios it supports, please refer to the [specification repo](https://github.com/microsoft/CorrelationVector).

### Features
#### Seed
Generates a new correlation vector.
```rust
CorrelationVector::new();
```
#### Extend
This adds a new counter in the vector clock.
```rust
let mut cv = CorrelationVector::new();
cv.extend();
```

#### Increment
This increments the latest counter in the vector clock.
```rust
let mut cv = CorrelationVector::new();
cv.increment();
```

#### Spin
This is the most complex function of the CorrelationVector. Spin changes the correlation vector such that the result should be unique and monotonically increasing without locking/atomic operations. It is used when the parent span is not able to atomically increment the vector clock for each child span.

```rust
let mut cv = CorrelationVector::new();
cv.spin();
```


### Explanation and example
The CorrelationVector contains a base-64 encoded uuid and a vector clock. The uuid is used to identify the vector clock and the vector clock is used to track the sequence of events.

```mermaid
sequenceDiagram
    participant A as Service A
    participant B as Service B
    participant C as Service C

    A -> A: A generates a correlation vector seed e.g. c3xEQzjqRlmr7zcQx9sBiQ.0
    A ->> B: A sends the message to B
    B -> B: B extends the correlation vector seed with a new event e.g. c3xEQzjqRlmr7zcQx9sBiQ.0.1
    B ->> C: B sends first message to C
    B -> B: B increments the cV when calling C again e.g. c3xEQzjqRlmr7zcQx9sBiQ.0.2
    B ->> C: B sends second message to C
```

This allows us to track the sequence of events in a complex distributed system.

### Disclaimer
I work for Microsoft and use CorrelationVectors but am not associated with the CorrelationVector team. This repo should *not* be considered a reference implementation of the CorrelationVector specification.