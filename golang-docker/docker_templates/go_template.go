package docker_templates

func GO_Write(file string) {

	var go_standard string = `FROM golang:latest
	
WORKDIR /go/src/app
	
COPY . .
	
RUN go mod download
	
RUN go build ` + searchFile("main.go") + ` .
	
EXPOSE 8080
	
CMD ["./main"]
`

	writeFile(file, go_standard)
	CREATE_SH("go-template")

}
