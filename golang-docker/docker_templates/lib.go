package docker_templates

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	"github.com/BurntSushi/toml"
)

func writeFile(file, data string) {
	os.WriteFile(file, []byte(data), 0644)
}

func CREATE_SH(name string) {
	rust_bat_string := `IMAGE_NAME="` + name + `"
IMAGE_TAG="latest"
	
docker build -t ${IMAGE_NAME}:${IMAGE_TAG} .`

	writeFile(name+".sh", rust_bat_string)

}

func findTomlName() string {
    file, err := os.Open("Cargo.toml")
    if err != nil {
        log.Fatal(err)
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