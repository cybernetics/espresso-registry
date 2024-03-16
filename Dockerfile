FROM alpine:3.19 as installrust

RUN apk update && apk add curl gcc build-base musl-dev openssl-dev
RUN curl -o /tmp/rustup-install.sh --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs
RUN sh /tmp/rustup-install.sh -y
ENV PATH=$PATH:/root/.cargo/env
RUN /root/.cargo/bin/rustup default stable

FROM installrust as build
RUN mkdir /espresso-registry
COPY . /espresso-registry
WORKDIR /espresso-registry
RUN /root/.cargo/bin/cargo build --release

FROM alpine:3.19 as runtime
COPY --from=build /espresso-registry/target/release/espresso-registry /usr/bin/espresso-registry
RUN chmod +x /usr/bin/espresso-registry
ENTRYPOINT [ "/usr/bin/espresso-registry" ]