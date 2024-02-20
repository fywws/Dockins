package docker_templates

import (
	"fmt"
	"log"
	"os"
	"path/filepath"
	"time"

	"github.com/BurntSushi/toml"
)

func writeFile(file, data string) {
	os.WriteFile(file, []byte(data), 0644)
}

func deleteFile(file string) {
    os.Remove(file)
}

func CREATE_SH(name string) {
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
docker build -f "$DockerfilePath/Dockerfile" -t "$ImageName" .`

	writeFile(name+".ps1", rust_bat_string)

}

func findTomlName() string {
    file, err := os.Open("Cargo.toml")
    if err != nil {
        fmt.Println( red + bold + " → File not found : Cargo.toml" + reset)
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

func searchFile(targetFile string) string {
    var resultPath string

    err := filepath.Walk(".", func(path string, info os.FileInfo, err error) error {
        if err != nil {
            return err
        }
        if !info.IsDir() && info.Name() == targetFile {
            resultPath, _ = filepath.Rel(".", path)
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
const colorRed = "\033[45m"
const red = "\x1b[31m"
const bold = "\x1b[1m"

func writePretty() {
	fmt.Print("[")
	repeatWithDelay(colorRed+"■"+reset, 2200, 29)
	fmt.Println("]")
}
func repeatWithDelay(character string, delay time.Duration, times int) {
	for i := 0; i < times; i++ {
		fmt.Print(character)
		time.Sleep(delay)
	}
}
