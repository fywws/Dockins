package docker_templates

import (
	"fmt"
	"os"
	"strconv"
)

func NODEJS_Write(makeS bool, port int) {
	portString := strconv.Itoa(port)

	var node_standard_npm string = `FROM node:latest

WORKDIR /app
	
COPY . /app
	
RUN npm install
	
EXPOSE `+portString+`:`+portString+`
	
CMD ["npm", "start"]`

	var node_standard_yarn string = `FROM node:latest

WORKDIR /app
		
COPY . /app
		
RUN npm install --global yarn
RUN yarn install --production

EXPOSE `+portString+`:`+portString+`
		
CMD ["yarn", "start"]`

// docker run --name appContainer -p 3000:3000 -it <imageID>

	fileFounded := false

	if fileExist("yarn.lock") && !fileExist("package.json") {
		fileFounded = true
		fmt.Println(bold +"Initializing Node-JS yarn docker..."+ reset)
		os.Create("Dockerfile")
		writePretty()
		writeFile("Dockerfile", node_standard_yarn)
		
	} else if fileExist("package.json") && !fileExist("yarn.lock") {
		fileFounded = true
		fmt.Println(bold +"Initializing Node-JS npm docker..."+ reset)
		os.Create("Dockerfile")
		writePretty()
		writeFile("Dockerfile", node_standard_npm)
	} else if !fileFounded {
		fmt.Println(red + bold + " × No node-js files are found in the directory" + reset)
	}

	if !makeS && fileFounded {
		CREATE_SH("nodejs-template", portString)
	}
}
