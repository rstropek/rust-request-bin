set dotenv-load

watch:
    watchexec -e rs,hbs -r -w ./ just run

run: (build)
    cargo run

build:
    cargo build

build-image:
    docker build --build-arg CONFIG=debug -t rstropek/request-bin .

push-image: (build-image)
    docker push rstropek/request-bin
