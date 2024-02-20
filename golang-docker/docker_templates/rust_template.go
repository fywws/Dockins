package docker_templates

import (
	"fmt"
	"os"
	"strconv"
)

func RUST_Write(makeS bool, port int) {
	portString := strconv.Itoa(port)
	var founded bool = false;
	toml := findTomlName()
	if toml != "" {
		founded = true
	}
	var rust_standard string = `FROM rust:latest	

WORKDIR /app
	
COPY . .
	
RUN cargo install --path .
RUN cargo build --release

EXPOSE `+portString+`:`+portString+`

CMD ["./target/debug/` + toml + `"]
`

	if (founded) {
		fmt.Println(bold +"Initializing Rust docker..."+ reset)
		
		os.Create("Dockerfile")
		writePretty()
		
		writeFile("Dockerfile", rust_standard)
		if !makeS {
			CREATE_SH("rust-template", portString)
		}
	} 
}
