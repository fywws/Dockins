package docker_templates

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func GO_Write(makeS bool, port int, db string) {
	portString := strconv.Itoa(port)


	file := searchFile("main.go")

	// gorm := StringInFile("gorm.io/gorm") // TODO: add support for gorm

	pq := StringInFile("go.mod", "github.com/lib/pq")
	pgx := StringInFile("go.mod", "github.com/jackc/pgx")

	var go_standard string =  `FROM golang:latest

WORKDIR /app

COPY . .

RUN go mod download
RUN go build -o main `+file+`

EXPOSE `+portString+`:`+portString+`

CMD ["`+strings.ReplaceAll(file, "main.go", "main")+`"]`


	var go_postgres string = `FROM golang:latest

WORKDIR /go/src/app

COPY go.mod go.sum ./

RUN go mod download

COPY . .

RUN go build -o /go/bin/app

FROM postgres:latest

ENV POSTGRES_USER user
ENV POSTGRES_PASSWORD password
ENV POSTGRES_DB dbname

COPY --from=0 /go/bin/app /app

EXPOSE `+portString+`:`+portString+`

CMD ["./app"]`



	
	if (file != ""){
		if (pq || pgx || db == "postgres") {
			fmt.Println(bold + "Initializing Go docker..." + reset )
			os.Create("Dockerfile")
			fmt.Println("\n", bold + "Change POSTGRES variables in Dockerfile" + reset)
			writePretty()
		
			writeFile("Dockerfile", go_postgres)
		} else {
			fmt.Println(bold + "Initializing Go docker..." + reset )
			os.Create("Dockerfile")
			writePretty()
			
			writeFile("Dockerfile", go_standard)
		}
		if !makeS {
			CREATE_SH("go-template", portString)
		}
	} else {
		fmt.Println( red+ bold + " × ERROR : main.go file not found" + reset)
	}

}
