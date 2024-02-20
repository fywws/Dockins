package docker_templates

import (
	"fmt"
	"os"
)

func RUST_Write(makeS bool) {
	var founded bool = false;
	toml := findTomlName()
	if toml != "" {
		founded = true
	}
	var rust_standard string = `FROM rust:latest	

WORKDIR /app
	
COPY . .
	
RUN cargo install .
RUN cargo build

CMD ["./target/debug/` + toml + `"]
`

	if (founded) {
		fmt.Println("Initializing Rust docker...")
		
		os.Create("Dockerfile")
		writePretty()
		
		writeFile("Dockerfile", rust_standard)
		if !makeS {
			CREATE_SH("rust-template")
		}
	} 
}
