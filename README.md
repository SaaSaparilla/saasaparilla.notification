# SaaSaparilla Notifications Service

### Acceptance Criteria

* service can be deployed in a sensible default configuration in an existing k8s cluster or locally (for testing 
  purposes) using:
  * `helm install`
  * `just run-kind`
* service supports client connections using sockets, sse, and http long polling
* service can scale horizontally sufficiently to handle 1M messages/second across 100M clients
* service exposes prometheus metrics
* service chart relies on the [prometheus-adapter](https://github.com/kubernetes-sigs/prometheus-adapter) for scaling metrics
* blast radius of any one service being temporarily unavailable for less than 10 minutes does not include data loss
* for the purpose of the above, failure to generate data is not considered data loss
* backing services incl. `kafka` and `garnet (redis)` can be deployed to k8s using manifest bundles _or_ plugged in
  using environment variables

### Installation Prerequisites

* [prometheus-adapter](https://github.com/kubernetes-sigs/prometheus-adapter)
* [metrics-server](https://github.com/kubernetes-sigs/metrics-server)
* [kube-prometheus-stack](https://github.com/prometheus-community/helm-charts/tree/main/charts/kube-prometheus-stack)

### Development

* `just lint`
* `just build`
* `just test`
* `just release`
* `just docker-build-all`
* `just run-kind`
* `just shutdown-kind`
* `just build-d2-diagrams`

#### Build Prerequisites

* [just](https://github.com/casey/just)
* [docker](https://docs.docker.com/reference/) or a compatible image builder (with [buildkit](https://docs.docker.com/build/buildkit/) support)
* [cargo](https://doc.rust-lang.org/cargo/)
* [rustc](https://doc.rust-lang.org/stable/book/)
* [clippy](https://github.com/rust-lang/rust-clippy#usage)
* [gcc](https://gcc.gnu.org/) or another C compiler

#### Test Prerequisites
* [kubectl](https://kubernetes.io/docs/reference/kubectl/)
* [kind](https://kind.sigs.k8s.io/)
* [flux](https://fluxcd.io/flux/)

### Architecture
#### Receiver

The receiver service is responsible for ensuring that:
* received notifications messages are shaped correctly
* any possible logical validations are performed to reject messages early
* logically valid and well-shaped notification messages are pushed to kafka

#### Director

The director service is responsible for ensuring that:
* consumed messages from kafka are sent to the distributor(s) holding the connection(s) to the receiving client(s)
* consumed messages that fail to reach their intended destination are retried
* consumed messages that fail after an appropriate number of retries are stored for retrieval in redis

#### Distributor
The distributor service is responsible for ensuring that:
* the client connection list in redis remains up to date
* forwarding messages received from the director service to the appropriate client(s)
* retrieving messages from redis and sending them to clients in the order received when a connection is established 
  BEFORE sending any new messages

![High Level Architecture Diagram](architecture/diagrams/Architecture.svg)
