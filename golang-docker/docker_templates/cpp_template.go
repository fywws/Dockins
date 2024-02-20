package docker_templates

import (
	"fmt"
	"os"
	"strconv"
)

func CPP_Write(makeS bool, port int) {
	portString := strconv.Itoa(port)

	var filename string = searchFile("main.cpp")

	var cpp_standard string = `FROM alpine:latest
	
RUN apk add --update ca-certificates curl make gcc musl-dev pkgconfig
	
COPY . /app
	
WORKDIR /app
	
RUN mkdir -p /app/build && cd /app/build && cmake .. && make
	
EXPOSE `+portString+`

CMD ["./bin/main.exe"]
`

	if (filename != "") {
		fmt.Println(bold +"Initializing C++ docker..."+ reset)
		
		os.Create("Dockerfile")
		writePretty()
		
		writeFile("Dockerfile", cpp_standard)
		if !makeS {
			CREATE_SH("cpp-template", portString)
		}
	} 
}
