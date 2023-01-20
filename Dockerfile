FROM rust:latest as build

RUN rustuop target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/personalwebsite
COPY . .

RUN cd frontend && trunk build --release
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/personalwebsite/targer/release/backend /usr/local/bin/backend

COPY --from=build /usr/src/personalwebsite/frontend/dist /usr/local/bin/dist

WORKDIR /usr/local/bin
CMD ["backend"]