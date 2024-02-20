package docker_templates

import (
	"fmt"
	"os"
)

func GO_Write(makeS bool) {

	file := searchFile("main.go")

	var go_standard string = `FROM golang:latest
	
WORKDIR /app
	
COPY . .
	
RUN go mod download
	
RUN go build ` + file + `
	
EXPOSE 8080
	
CMD ["./main"]
`

	if (file != ""){
		fmt.Println("Initializing Go docker...")
		os.Create("Dockerfile")
		writePretty()
		
		writeFile("Dockerfile", go_standard)
		if !makeS {
			CREATE_SH("go-template")
		}
	} else {
		fmt.Println( red+ bold + " → ERROR : main.go file not found" + reset)
	}

}
