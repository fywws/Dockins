package docker_templates

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func GO_Write(makeS bool, port int, db string) {

	var username string;
	var password string;
	var database string;


	portString := strconv.Itoa(port)


	file := searchFile("main.go")

	// gorm := StringInFile("gorm.io/gorm") // TODO: add support for gorm

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

COPY --from=0 /go/bin/app /app

EXPOSE `+portString+`:`+portString+`

CMD ["./app"]`


if db != "" && db == "postgres" {
	fmt.Println(bold + "PostgreSQL Username? " + reset)
	fmt.Print(bold + "» " + reset)
	fmt.Scan(&username)

	fmt.Println(bold + "Username Password? " + reset)
	fmt.Print(bold +"» " + reset)
	fmt.Scan(&password)

	fmt.Println(bold + "PostgreSQL Database name? " + reset)
	fmt.Print(bold +"» " + reset)
	fmt.Scan(&database)

	fmt.Println("\n"+  green + bold + "Change PostgreSQL config in config/ ..." + reset + "\n")

}

var postgres_contrainer string = `FROM postgres:latest

ARG PGDATA=/var/lib/postgresql/data

ENV POSTGRES_USER="`+username+`"
ENV POSTGRES_PASSWORD="`+ password + `"
ENV POSTGRES_DB="`+database+`"

RUN mkdir -p ${PGDATA} && chown postgres:postgres ${PGDATA}

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
EXPOSE 5432`

postgres_config := `#-----------------------------
# Data Directory Settings
#-----------------------------

data_directory = '/var/lib/postgresql/data'`

postgres_entrypoint := `#!/bin/bash
set -e

# Wait for PostgreSQL service to become available
until pg_isready -q -h localhost -p 5432 ; do
sleep 1
done

# Alter the user role to superuser
echo "ALTER USER $POSTGRES_USER WITH SUPERUSER;" | psql -U "$POSTGRES_USER"

# Run the PostgreSQL service
exec gosu postgres "$@"`

	
	if (file != ""){

		if db != "" {
			if db == "postgres" && username != "" && password != "" && database != "" {
				fmt.Println(bold + "Initializing Go docker..." + reset )
				os.Create("Dockerfile")
				writePretty()
		
				writeFile("Dockerfile", go_postgres)

				writeFile("postgres.Dockerfile", postgres_contrainer)
				os.Mkdir("config", 0755)
				writeFile("config/postgresql.conf", postgres_config)
				writeFile("entrypoint.sh", postgres_entrypoint)
			} else {
				fmt.Println( red + bold + " × ERROR : provided wrong" + reset)
				os.Remove("postgres.Dockerfile")
			}
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
