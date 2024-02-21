package docker_templates

import (
	"fmt"
	"os"
	"strconv"
)

func RUST_Write(makeS bool, port int, db string) {

	var username string;
	var password string;
	var database string;


	portString := strconv.Itoa(port)
	var founded bool = false;
	toml := findTomlName()
	if toml != "" {
		founded = true
	}
	var rust_standard string = `FROM rust:latest
COPY ./ ./

RUN cargo build --release

EXPOSE `+portString+`:`+portString+`

CMD ["./target/release/`+toml+`"]
`



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

	if db != "" {
		if db == "postgres" && username != "" && password != "" && database != "" {
			writeFile("postgres.Dockerfile", postgres_contrainer)
			os.Mkdir("config", 0755)
			writeFile("config/postgresql.conf", postgres_config)
			writeFile("entrypoint.sh", postgres_entrypoint)
		} else {
			fmt.Println( red + bold + " × ERROR : provided wrong" + reset)
			os.Remove("postgres.Dockerfile")
		}
	}


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
