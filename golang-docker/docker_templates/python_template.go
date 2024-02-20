package docker_templates

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
)

func PY_Write(makeS bool, port int) {
	portString := strconv.Itoa(port)

	var filename string = searchFile("main.py")
	varName, _ := GetAppNameFromFile(filename); 
	content, _ := ioutil.ReadFile(filename)
	var isFastApi bool = findAppWithFastAPI(string(content));

	var py_standard string = `FROM python

COPY . /app

WORKDIR /app

RUN pip install -r requirements.txt

EXPOSE `+portString+`:`+portString+`

CMD ["python", "`+filename+`"]`

	var py_uvicorn string = `FROM python
	
COPY . /app

WORKDIR /app

RUN pip install -r requirements.txt
RUN pip install uvicorn

EXPOSE `+portString+`:`+portString+`

CMD ["uvicorn", "`+filename+":"+varName+`", "--port", "`+portString+`"]`


	if (filename != "") {
		if (isFastApi) {
			fmt.Println(bold +"Initializing Python FastApi docker..."+ reset)
		
			os.Create("Dockerfile")
			writePretty()
		
			writeFile("Dockerfile", py_uvicorn)
		} else {

			fmt.Println(bold +"Initializing Python docker..."+ reset)
			
			os.Create("Dockerfile")
			writePretty()
			
			writeFile("Dockerfile", py_standard)
		}
		if !makeS {
			CREATE_SH("py-template", portString)
		}
	} else {
		fmt.Println( red+ bold + " × ERROR : main.py file not found" + reset)
	}
}
