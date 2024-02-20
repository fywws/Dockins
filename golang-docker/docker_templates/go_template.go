package docker_templates

import (
	"fmt"
	"os"
	"strings"
)

func GO_Write(makeS bool) {

	file := searchFile("main.go")

	var go_standard string = `FROM golang:latest

WORKDIR /app

COPY . .

RUN go mod download
RUN go build -o main `+file+`

EXPOSE 8080:8080

CMD ["`+strings.ReplaceAll(file, "main.go", "main")+`"]`
	
	if (file != ""){
		fmt.Println(bold + "Initializing Go docker..." + reset )
		os.Create("Dockerfile")
		writePretty()
		
		writeFile("Dockerfile", go_standard)
		if !makeS {
			CREATE_SH("go-template")
		}
	} else {
		fmt.Println( red+ bold + " × ERROR : main.go file not found" + reset)
	}

}
