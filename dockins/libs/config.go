package libsa

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

func RUST_Write(makeS bool, port int, db string) {

	var username string
	var password string
	var database string

	portString := strconv.Itoa(port)
	var founded bool = false
	toml, _ := findTomlName()
	if toml != "" {
		founded = true
	}
	var rust_standard string = `FROM rust:latest
COPY ./ ./

RUN cargo build --release

EXPOSE ` + portString + `:` + portString + `

CMD ["./target/release/` + toml + `"]
`

	if db != "" && db == "postgres" {

		fmt.Println(bold + "PostgreSQL Username? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&username)

		fmt.Println(bold + "Username Password? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&password)

		fmt.Println(bold + "PostgreSQL Database name? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&database)

		fmt.Println("\n" + green + bold + "Change PostgreSQL config in config/ ..." + reset + "\n")

	}

	var postgres_contrainer string = `FROM postgres:latest

ARG PGDATA=/var/lib/postgresql/data

ENV POSTGRES_USER="` + username + `"
ENV POSTGRES_PASSWORD="` + password + `"
ENV POSTGRES_DB="` + database + `"

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
			fmt.Println(red + bold + " × ERROR : provided wrong" + reset)
			os.Remove("postgres.Dockerfile")
		}
	}

	if founded {
		fmt.Println(bold + "Initializing Rust docker..." + reset)

		os.Create("Dockerfile")
		writePretty()

		writeFile("Dockerfile", rust_standard)
		if !makeS {
			CreateSh("rust-template", portString)
		}
	}
}

func NODEJS_Write(makeS bool, port int, db string) {

	var username string
	var password string
	var database string

	portString := strconv.Itoa(port)

	var node_standard_npm string = `FROM node:latest

WORKDIR /app
	
COPY . /app
	
RUN npm install
	
EXPOSE ` + portString + `:` + portString + `
	
CMD ["npm", "start"]`

	var node_standard_yarn string = `FROM node:latest

WORKDIR /app
		
COPY . /app
		
RUN npm install --global yarn
RUN yarn install --production

EXPOSE ` + portString + `:` + portString + `
		
CMD ["yarn", "start"]`

	if db != "" && db == "postgres" {

		fmt.Println(bold + "PostgreSQL Username? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&username)

		fmt.Println(bold + "Username Password? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&password)

		fmt.Println(bold + "PostgreSQL Database name? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&database)

		fmt.Println("\n" + green + bold + "Change PostgreSQL config in config/ ..." + reset + "\n")

	}

	var postgres_contrainer string = `FROM postgres:latest

ARG PGDATA=/var/lib/postgresql/data

ENV POSTGRES_USER="` + username + `"
ENV POSTGRES_PASSWORD="` + password + `"
ENV POSTGRES_DB="` + database + `"

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

	// docker run --name appContainer -p 3000:3000 -it <imageID>

	fileFounded := false

	if db != "" {
		if db == "postgres" && username != "" && password != "" && database != "" {
			writeFile("postgres.Dockerfile", postgres_contrainer)
			os.Mkdir("config", 0755)
			writeFile("config/postgresql.conf", postgres_config)
			writeFile("entrypoint.sh", postgres_entrypoint)
		} else {
			fmt.Println(red + bold + " × ERROR : provided wrong" + reset)
			os.Remove("postgres.Dockerfile")
		}
	}

	if fileExist("yarn.lock") && !fileExist("package.json") {
		fileFounded = true
		fmt.Println(bold + "Initializing Node-JS yarn docker..." + reset)
		os.Create("Dockerfile")
		writePretty()
		writeFile("Dockerfile", node_standard_yarn)

	} else if fileExist("package.json") && !fileExist("yarn.lock") {
		fileFounded = true
		fmt.Println(bold + "Initializing Node-JS npm docker..." + reset)
		os.Create("Dockerfile")
		writePretty()
		writeFile("Dockerfile", node_standard_npm)
	} else if !fileFounded {
		fmt.Println(red + bold + " × No node-js files are found in the directory" + reset)
	}

	if !makeS && fileFounded {
		CreateSh("nodejs-template", portString)
	}
}

func PY_Write(makeS bool, port int, db string) {

	var username string
	var password string
	var database string

	portString := strconv.Itoa(port)

	var filename string = searchFile("main.py")
	varName, _ := GetAppNameFromFile(filename)
	content, _ := ioutil.ReadFile(filename)
	var isFastApi bool = findAppWithFastAPI(string(content))

	var py_standard string = `FROM python

COPY . /app

WORKDIR /app

RUN pip install -r requirements.txt

EXPOSE ` + portString + `:` + portString + `

CMD ["python", "` + filename + `"]`

	var py_uvicorn string = `FROM python
	
COPY . /app

WORKDIR /app

RUN pip install -r requirements.txt
RUN pip install uvicorn

EXPOSE ` + portString + `:` + portString + `

CMD ["uvicorn", "` + filename + ":" + varName + `", "--port", "` + portString + `"]`

	if db != "" && db == "postgres" {

		fmt.Println(bold + "PostgreSQL Username? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&username)

		fmt.Println(bold + "Username Password? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&password)

		fmt.Println(bold + "PostgreSQL Database name? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&database)

		fmt.Println("\n" + green + bold + "Change PostgreSQL config in config/ ..." + reset + "\n")

	}

	var postgres_contrainer string = `FROM postgres:latest

ARG PGDATA=/var/lib/postgresql/data

ENV POSTGRES_USER="` + username + `"
ENV POSTGRES_PASSWORD="` + password + `"
ENV POSTGRES_DB="` + database + `"

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

	if filename != "" {

		if db != "" {
			if db == "postgres" && username != "" && password != "" && database != "" {
				writeFile("postgres.Dockerfile", postgres_contrainer)
				os.Mkdir("config", 0755)
				writeFile("config/postgresql.conf", postgres_config)
				writeFile("entrypoint.sh", postgres_entrypoint)
			} else {
				fmt.Println(red + bold + " × ERROR : provided wrong" + reset)
				os.Remove("postgres.Dockerfile")
			}
		}

		if isFastApi {
			fmt.Println(bold + "Initializing Python FastApi docker..." + reset)

			os.Create("Dockerfile")
			writePretty()

			writeFile("Dockerfile", py_uvicorn)
		} else {

			fmt.Println(bold + "Initializing Python docker..." + reset)

			os.Create("Dockerfile")
			writePretty()

			writeFile("Dockerfile", py_standard)
		}

		if !makeS {
			CreateSh("py-template", portString)
		}
	} else {
		fmt.Println(red + bold + " × ERROR : main.py file not found" + reset)
	}
}

func GO_Write(makeS bool, port int, db string) {

	var username string
	var password string
	var database string

	portString := strconv.Itoa(port)

	file := searchFile("main.go")

	// gorm := StringInFile("gorm.io/gorm") // TODO: add support for gorm

	var go_standard string = `FROM golang:latest

WORKDIR /app

COPY . .

RUN go mod download
RUN go build -o main ` + file + `

EXPOSE ` + portString + `:` + portString + `

CMD ["` + strings.ReplaceAll(file, "main.go", "main") + `"]`

	var go_postgres string = `FROM golang:latest

WORKDIR /go/src/app

COPY go.mod go.sum ./

RUN go mod download

COPY . .

RUN go build -o /go/bin/app

COPY --from=0 /go/bin/app /app

EXPOSE ` + portString + `:` + portString + `

CMD ["./app"]`

	if db != "" && db == "postgres" {
		fmt.Println(bold + "PostgreSQL Username? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&username)

		fmt.Println(bold + "Username Password? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&password)

		fmt.Println(bold + "PostgreSQL Database name? " + reset)
		fmt.Print(bold + "» " + reset)
		fmt.Scan(&database)

		fmt.Println("\n" + green + bold + "Change PostgreSQL config in config/ ..." + reset + "\n")

	}

	var postgres_contrainer string = `FROM postgres:latest

ARG PGDATA=/var/lib/postgresql/data

ENV POSTGRES_USER="` + username + `"
ENV POSTGRES_PASSWORD="` + password + `"
ENV POSTGRES_DB="` + database + `"

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

	if file != "" {

		if db != "" {
			if db == "postgres" && username != "" && password != "" && database != "" {
				fmt.Println(bold + "Initializing Go docker..." + reset)
				os.Create("Dockerfile")
				writePretty()

				writeFile("Dockerfile", go_postgres)

				writeFile("postgres.Dockerfile", postgres_contrainer)
				os.Mkdir("config", 0755)
				writeFile("config/postgresql.conf", postgres_config)
				writeFile("entrypoint.sh", postgres_entrypoint)
			} else {
				fmt.Println(red + bold + " × ERROR : provided wrong" + reset)
				os.Remove("postgres.Dockerfile")
			}
		} else {
			fmt.Println(bold + "Initializing Go docker..." + reset)
			os.Create("Dockerfile")
			writePretty()

			writeFile("Dockerfile", go_standard)
		}
		if !makeS {
			CreateSh("go-template", portString)
		}
	} else {
		fmt.Println(red + bold + " × ERROR : main.go file not found" + reset)
	}

}
