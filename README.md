# SaaSaparilla Notifications Service

### Acceptance Criteria

* service can be deployed in a sensible default configuration using:
  * `helm install`
* service supports client connections using sockets, sse, and http long polling
* service can scale horizontally sufficiently to handle 1M messages/second across 100M clients
* service exposes prometheus metrics
* service chart relies on the [prometheus-adapter](https://github.com/kubernetes-sigs/prometheus-adapter) for scaling metrics
* blast radius of any service being temporarily unavailable for less than 10 minutes does not include data loss

### Installation Prerequisites

* [prometheus-adapter](https://github.com/kubernetes-sigs/prometheus-adapter)
* [metrics-server](https://github.com/kubernetes-sigs/metrics-server)
* [kube-prometheus-stack](https://github.com/prometheus-community/helm-charts/tree/main/charts/kube-prometheus-stack)

### Development

* `cargo fmt`
* `cargo build`
* `cargo test`
* `cargo clippy`
* `cargo run`
* `docker build .`

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
* consumed messages that fail after an appropriate number of retries are stored for retrieval in postgres

#### Distributor
The distributor service is responsible for ensuring that:
* the client connection list in redis remains up to date
* forwarding messages received from the director service to the appropriate client(s)
* retrieving messages from postgres and sending them to clients in the order received when a connection is established BEFORE sending any new messages