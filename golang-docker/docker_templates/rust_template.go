package docker_templates

func RUST_Write(file string) {

	var go_standard string = `FROM rust:latest as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/app/target/release/your_app_name /usr/local/bin/your_app_name

CMD ["your_app_name"]
`

	writeFile(file, go_standard)
	CREATE_SH("rust-template")

}
