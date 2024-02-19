package docker_templates

func RUST_Write(file string) {

	var go_standard string = `FROM rust:latest

WORKDIR /app
	
COPY . .
	
RUN cargo install --path .
	
CMD ["target/debug/` + findTomlName() + `"]
`

	writeFile(file, go_standard)
	CREATE_SH("rust-template")

}