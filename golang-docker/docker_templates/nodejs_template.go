package docker_templates

import (
	"fmt"
	"os"
)

func NODEJS_Write(makeS bool) {
	var node_standard_npm string = `FROM node:latest

WORKDIR /app
	
COPY . /app
	
RUN npm install
	
EXPOSE 3000:3000
	
CMD ["npm", "start"]`

	var node_standard_yarn string = `FROM node:latest

WORKDIR /app
		
COPY . /app
		
RUN npm install --global yarn
RUN yarn install --production

EXPOSE 3000:3000
		
CMD ["yarn", "start"]`

// docker run --name appContainer -p 3000:3000 -it <imageID>

	fileFounded := false

	if fileExist("yarn.lock") && !fileExist("package.json") {
		fileFounded = true
		fmt.Println("Initializing Node-JS yarn docker...")
		os.Create("Dockerfile")
		writePretty()
		writeFile("Dockerfile", node_standard_yarn)
		
	} else if fileExist("package.json") && !fileExist("yarn.lock") {
		fileFounded = true
		fmt.Println("Initializing Node-JS npm docker...")
		os.Create("Dockerfile")
		writePretty()
		writeFile("Dockerfile", node_standard_npm)
	} else if !fileFounded {
		fmt.Println(red + bold + " × No node-js files are found in the directory" + reset)
	}

	if !makeS && fileFounded {
		CREATE_SH("nodejs-template")
	}
}
