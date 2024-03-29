package docker_templates

import (
	"fmt"
	"io/ioutil"
	"log"
	lib "main/libs/progress"
	"os"
	"path/filepath"
	"regexp"
	"strings"

	"github.com/BurntSushi/toml"
)

func writeFile(file, data string) {
	os.WriteFile(file, []byte(data), 0644)
}

func deleteFile(file string) {
	os.Remove(file)
}

func CREATE_SH(name string, port string) {
	rust_bat_string := `param(
    [Parameter(Mandatory=$true)]
    [String]$ImageName,

    [Parameter()]
    [String]$DockerfilePath
)

if (-not $DockerfilePath) {
    $DockerfilePath = "."
}

Write-Host "Building Docker image '$ImageName'..."
docker build -f "$DockerfilePath/Dockerfile" -t "$ImageName" .

Write-Host "Starting Docker container '$ImageName'..."
docker run --name appContainer -p ` + string(port) + `:` + string(port) + ` -it $ImageName`

	writeFile(name+".ps1", rust_bat_string)

}

func findTomlName() string {
	file, err := os.Open("Cargo.toml")
	if err != nil {
		fmt.Println(red + bold + " × File not found : Cargo.toml" + reset)
	}
	defer file.Close()

	var data map[string]interface{}

	if _, err := toml.DecodeReader(file, &data); err != nil {
		log.Fatal(err)
	}

	name, ok := data["package"].(map[string]interface{})["name"].(string)
	if !ok {
		log.Fatal("name field not found or not a string")
	}

	return name
}

func fileExist(fileName string) bool {
	_, err := os.Stat(fileName)
	if os.IsNotExist(err) {
		return false
	}
	return true
}

func searchFile(targetFile string) string {
	var resultPath string

	err := filepath.Walk(".", func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && info.Name() == targetFile {
			resultPath, _ = filepath.Rel(".", path)
			resultPath = filepath.ToSlash(resultPath) // Преобразование пути
			return fmt.Errorf("file found")
		}
		return nil
	})

	if err != nil && err.Error() != "file found" {
		return ""
	}

	return resultPath
}

const (
	reset = "\033[0m"
	red   = "\x1b[31m"
	green = "\x1b[32m"
	bold  = "\x1b[1m"
	blue  = "\x1b[96m"
)

func writePretty() {

	lib.ProgressAnimated()

	fmt.Println("\n", green+bold+"Happy hacking"+reset)
}

func StringInFile(filename string, searchStr string) bool {
	data, err := ioutil.ReadFile(filename)
	if err != nil {
		return false
	}

	return strings.Contains(string(data), searchStr)
}

func findAppWithFastAPI(code string) bool {
	pattern := regexp.MustCompile(`([A-Za-z_][A-Za-z0-9_]*)\s*=\s*FastAPI\(`)
	matches := pattern.FindAllStringSubmatch(code, -1)

	for _, match := range matches {
		if strings.ToLower(match[1]) == "app" {
			return true
		}
	}

	return false
}

func GetAppName(code string) (name string, ok bool) {
	lines := strings.Split(code, "\n")

	for _, line := range lines {

		trimmedLine := strings.TrimSpace(line)
		if strings.Contains(trimmedLine, "= FastAPI()") {
			nameParts := strings.SplitN(trimmedLine, "=", 2)
			return strings.TrimSpace(nameParts[0]), true
		}
	}
	return "", false
}

func GetAppNameFromFile(filename string) (name string, ok bool) {
	contentBytes, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	contentString := string(contentBytes)

	return GetAppName(contentString)
}
