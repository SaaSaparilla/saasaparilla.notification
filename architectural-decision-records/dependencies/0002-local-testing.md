# Use kind to bootstrap a local cluster for testing

## Context and Problem Statement

Which kafka middleware do we use?

## Considered Options

* cargo test
* custom scripts
* docker-compose
* kind

### cargo test

###### Pros

* native tool
* this will still be used as a gate before kind testing

###### Cons

* no way to test helm charts, kustomizations, timoni bundles, etc.

### custom scripts

###### Pros

* complete control over the execution
* these will be 

###### Cons

* no adoption
* extra documentation required
* testing helm charts, kustomizations, timoni bundles, etc. can be done but is often insufficient due to the stringly 
  typed nature of these tools
* integration testing can be done on the binaries, but not on the helm charts, kustomizations, timoni bundles, etc.
* posix-compatible shells are kludgy and gross, and nushell doesn't have wide adoption
* bsd/gnu incompatibilities
* potentially high maintenance burden
* "It works on my machine"

### docker-compose

###### Pros

* fairly simple to create
* wide adoption

###### Cons

* split brain between local testing and dev/production deployments
* everything that I am designing is meant to be k8s-native, and docker-compose is not
* no way to test helm charts, kustomizations, timoni bundles, etc.

### kind

###### Pros

* high replicability
* prod-like shape
* supports all levels of testing, including deployment bundles
* can be split into a separate project `SaaSaparilla.bootstrap`
* project is maintained by the k8s team for testing k8s releases, so it will be maintained regardless of adoption

###### Cons

* cluster upgrades are not supported
* high setup cost
* limited adoption as of yet

## Decision Outcome

Use `kind` to bootstrap a cluster and package that bootstrapped mvp cluster as `SaaSaparilla.bootstrap` for use
across all projects in the `SaaSaparilla` umbrella.