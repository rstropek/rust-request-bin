set dotenv-load

# Watch sources and re-run "cargo run" on changes
# Use this task for local development.
watch:
    watchexec -e rs,hbs -r -w ./ just run

# Build and run app locally
run: (build)
    cargo run

# Build app locally (debug build)
build:
    cargo build

# Run tests locally
test:
    cargo test

# Build container images locally
build-image:
    docker build -t rstropek/request-bin --target production .
    docker build -t rstropek/request-bin-tests --target tests .

# Run tests locally in container
run-docker-tests:
    docker run -t --rm -w /app rstropek/request-bin-tests cargo test --release

# Push container image to Docker Hub
push-image: (build-image)
    docker push rstropek/request-bin

# Build container image locally, tag for ACR
build-image-acr:
    docker build \
        --build-arg BASE=acrbeyondimages.azurecr.io/rust:alpine \
        -t acrbeyondimages.azurecr.io/request-bin \
        --target production \
        .

# Push locally built image to ACR
push-image-acr: (build-image-acr)
    docker push acrbeyondimages.azurecr.io/request-bin

# Build container image in ACR (quick task)
build-image-acr-cloud:
    az acr build \
        --registry acrbeyondimages \
        --build-arg BASE=acrbeyondimages.azurecr.io/rust:alpine \
        -t acrbeyondimages.azurecr.io/request-bin \
        --target production \
        .
