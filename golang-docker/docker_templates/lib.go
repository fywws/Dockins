package docker_templates

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"time"
	"unicode"

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
docker run --name appContainer -p `+string(port)+`:`+string(port)+` -it $ImageName`

	writeFile(name+".ps1", rust_bat_string)

}

func findTomlName() string {
    file, err := os.Open("Cargo.toml")
    if err != nil {
        fmt.Println( red + bold + " × File not found : Cargo.toml" + reset)
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



const reset = "\033[0m"
const red = "\x1b[31m"
const green = "\x1b[32m"
const bold = "\x1b[1m"
const blue = "\x1b[96m"

func writePretty() {
	fmt.Print(bold + "[" + reset)
	repeatWithDelay(bold + blue + "■"+reset, 2600, 29)
	fmt.Println(bold+"]"+ reset)

    fmt.Println("\n", green + bold + "Happy hacking!" + reset)
}

func repeatWithDelay(character string, delay time.Duration, times int) {
	for i := 0; i < times; i++ {
		fmt.Print(character)
		time.Sleep(delay)
	}
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

func extractVariableNameFromLine(line string) string {
	parts := strings.SplitN(line, "=", 2)
	if len(parts) != 2 {
		return ""
	}

	assignmentParts := strings.FieldsFunc(parts[0], func(r rune) bool { return !unicode.IsLetter(r) })
	if len(assignmentParts) <= 1 || assignmentParts[len(assignmentParts)-1] != "FastAPI()" {
		return ""
	}

	return assignmentParts[0]
}


func GetAppName(code string) (name string, ok bool) {
    // Split the code into individual lines
    lines := strings.Split(code, "\n")
    
    // Iterate through all lines
    for _, line := range lines {
        
        // Trim leading and trailing spaces
        trimmedLine := strings.TrimSpace(line)
            
            // Check if the line matches the pattern
            if strings.Contains(trimmedLine, "= FastAPI()") {
                // Extract the name
                nameParts := strings.SplitN(trimmedLine, "=", 2)
                
                // Remove extra spaces from the extracted name
                return strings.TrimSpace(nameParts[0]), true
            }
    }
    // Return empty name and false flag if no match found
    return "", false
}

func GetAppNameFromFile(filename string) (name string, ok bool) {
    // Read the entire content of the file
    contentBytes, err := os.ReadFile(filename)
    if err != nil {
        log.Fatal(err)
    }
    
    // Convert the bytes to a string representation
    contentString := string(contentBytes)
    
    // Call the existing GetAppName function with the string content
    return GetAppName(contentString)
}