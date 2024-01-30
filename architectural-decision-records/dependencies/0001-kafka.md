# Use rust-rdkafka for kafka clients

## Context and Problem Statement

Which kafka middleware do we use?

## Considered Options

* [kafka-rust](https://github.com/kafka-rust/kafka-rust)
* [rust-rdkafka](https://github.com/fede1024/rust-rdkafka)

### kafka-rust

###### Pros

* cross-platform compilation with cargo

###### Cons

* missing some critical features
  * partial success reporting of batch writes
  * zookeeperless kraft support

### rust-rdkafka

###### Pros

* as a wrapper around `librdkakfa`, support for new features arrives quickly

###### Cons

* adds some complexity to compilation because it depends on cmake
* platform support may be limited
* statically including librdkafka makes the resulting binary quite large because cargo cannot do any tree shaking or 
  dependency elision

## Decision Outcome

Use `rust-rdkafka` for the initial buildout, and switch to a rust-native solution once one is sufficiently mature.