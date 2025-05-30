default:
    just --list

lint:
    echo "INFO:    Linting..."
    cargo fmt
    cargo clippy

build:
    echo "INFO:    Building..."
    cargo build --future-incompat-report --color always

test:
    echo "INFO:    Testing..."
    cargo test

release: lint build test
    echo "INFO:    Creating release build..."
    cargo build --future-incompat-report --color always --release --locked

run-director:
    cargo run --bin saasaparilla-notification-director -- --config-file-path=crates/director/config.toml

run-distrubutor:
    cargo run --bin saasaparilla-notification-distrubutor -- --config-file-path=crates/distrubutor/config.toml

run-receiver:
    cargo run --bin saasaparilla-notification-receiver -- --config-file-path=crates/receiver/config.toml

docker-build-all: docker-build-dependencies docker-build-director docker-build-distributor docker-build-receiver

docker-build-dependencies:
    echo "INFO:    Building dependencies..."
    docker buildx build . --file ./docker/Dockerfile --target dependency_builder

docker-build-director:
    echo "INFO:    Building director image..."
    docker buildx build . --file ./docker/Dockerfile --target minimal --build-arg COMPONENT=director -t saasaparilla/notification/director:latest -t localhost:5000/saasaparilla/notification/director:latest
    docker push localhost:5000/saasaparilla/notification/director:latest

docker-build-distributor:
    echo "INFO:    Building distributor image..."
    docker buildx build . --file ./docker/Dockerfile --target minimal --build-arg COMPONENT=distributor -t saasaparilla/notification/distributor:latest -t localhost:5000/saasaparilla/notification/distributor:latest
    docker push localhost:5000/saasaparilla/notification/distributor:latest

docker-build-receiver:
    echo "INFO:    Building receiver image..."
    docker buildx build . --file ./docker/Dockerfile --target minimal --build-arg COMPONENT=receiver -t saasaparilla/notification/receiver:latest -t localhost:5000/saasaparilla/notification/receiver:latest
    docker push localhost:5000/saasaparilla/notification/receiver:latest

run-docker: docker-build-all
    echo "INFO:    Running locally..."
    docker-compose --file ./docker/docker-compose.yaml down
    docker-compose --file ./docker/docker-compose.yaml up
    docker-compose --file ./docker/docker-compose.yaml down

run-kind:
    echo "INFO:    Deploying kind cluster..."
    kind create cluster --config kind/cluster.yaml --name saasaparilla-notification --wait 5m
    docker run --name saasaparilla-notification-cloud-provider-kind --rm --detach --network kind -v /var/run/docker.sock:/var/run/docker.sock registry.k8s.io/cloud-provider-kind/cloud-controller-manager:v0.6.0
    docker run --name saasaparilla-notification-registry --rm --detach --network kind -p 5000:5000 registry:2
    kubectl --context kind-saasaparilla-notification apply -k kind/bootstrap/ ||\
    kubectl --context kind-saasaparilla-notification apply -k kind/bootstrap/ #do this twice to apply the custom resources
    kubectl --context kind-saasaparilla-notification wait --for=create namespace/ingress-nginx
    kubectl --context kind-saasaparilla-notification wait --for=create service/ingress-nginx-controller --namespace ingress-nginx
    kubectl --context kind-saasaparilla-notification wait --for=jsonpath='{.status.loadBalancer.ingress[0].ip}' service/ingress-nginx-controller --namespace ingress-nginx
    echo "INFO:    Kind cluster is ready to use"

generate-flux-system-yaml:
    flux install --context kind-saasaparilla-notification --namespace=flux-system --watch-all-namespaces=false --export > kind/bootstrap/flux-install.yaml

shutdown-kind:
    kind delete cluster --name saasaparilla-notification
    docker stop saasaparilla-notification-cloud-provider-kind || echo "WARNING: Docker container saasaparilla-notification-cloud-provider-kind not found.  Continuing..."
    docker stop saasaparilla-notification-registry || echo "WARNING: Docker container saasaparilla-notification-registry not found.  Continuing..."
    docker stop $(docker container list -f "label=io.x-k8s.cloud-provider-kind.cluster=saasaparilla-notification" --format '{{{{ json .Names}}' | tr -d '"') || echo "WARNING: Unable to stop kindccm container(s).  Continuing..."

build-d2-diagrams:
    d2 architecture/diagrams.d2

