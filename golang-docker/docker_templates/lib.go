package docker_templates

import "os"

func writeFile(file, data string) {
	os.WriteFile(file, []byte(data), 0644)
}

func CREATE_SH(name string) {
	rust_bat_string := `IMAGE_NAME="` + name + `"
IMAGE_TAG="latest"
	
docker build -t ${IMAGE_NAME}:${IMAGE_TAG} .`

	writeFile(name+".sh", rust_bat_string)

}