default: lint build test

lint:
  echo Linting...
  cargo fmt
  cargo clippy

build:
  echo Building...
  cargo build --future-incompat-report --color always

test:
  echo Testing...
  cargo test

release: lint build test
  echo Creating release build...
  cargo build --future-incompat-report --color always --release --locked

docker-build-all: docker-build-director docker-build-distributor docker-build-receiver

docker-build-director:
  echo Building director image...
  docker buildx build --build-arg COMPONENT=director . -t director:latest

docker-build-distributor:
  echo Building distributor image...
  docker buildx build --build-arg COMPONENT=distributor . -t distributor:latest

docker-build-receiver:
  echo Building receiver image...
  docker buildx build --build-arg COMPONENT=receiver . -t receiver:latest

docker-run: docker-build-all
  echo Running locally...
  docker run -p 3000 receiver:latest
  docker run -p 3001 director:latest
  docker run -p 3002 director:latest
