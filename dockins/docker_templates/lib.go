package docker_templates

import (
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"time"
	"unicode"

	lib "main/libs/progress" // Assuming this package is located correctly

	"github.com/BurntSushi/toml"
)

// Constants for color formatting
const (
	reset = "\033[0m"
	red   = "\x1b[31m"
	green = "\x1b[32m"
	bold  = "\x1b[1m"
	blue  = "\x1b[96m"
)

// writeFile writes data to a file
func writeFile(file, data string) error {
	return os.WriteFile(file, []byte(data), 0644)
}

// deleteFile deletes a file
func deleteFile(file string) error {
	return os.Remove(file)
}

// CreateSh creates a PowerShell script
func CreateSh(name string, port string) {
	// Implementation
}

// findTomlName finds the name from a Cargo.toml file
func findTomlName() (string, error) {
	file, err := os.Open("Cargo.toml")
	if err != nil {
		return "", fmt.Errorf(red + bold + " × File not found : Cargo.toml" + reset)
	}
	defer file.Close()

	var data map[string]interface{}

	if _, err := toml.DecodeReader(file, &data); err != nil {
		return "", err
	}

	name, ok := data["package"].(map[string]interface{})["name"].(string)
	if !ok {
		return "", fmt.Errorf("name field not found or not a string")
	}

	return name, nil
}

// fileExist checks if a file exists
func fileExist(fileName string) bool {
	_, err := os.Stat(fileName)
	return !os.IsNotExist(err)
}

// searchFile searches for a file
func searchFile(targetFile string) string {
	var resultPath string

	err := filepath.Walk(".", func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && info.Name() == targetFile {
			resultPath, _ = filepath.Rel(".", path)
			resultPath = filepath.ToSlash(resultPath) // Convert path
			return fmt.Errorf("file found")
		}
		return nil
	})

	if err != nil && err.Error() != "file found" {
		return ""
	}

	return resultPath
}

// writePretty writes pretty output
func writePretty() {
	lib.ProgressAnimated()
	fmt.Println("\n", green+bold+"Happy hacking"+reset)
}

// repeatWithDelay repeats a character with a delay
func repeatWithDelay(character string, delay time.Duration, times int) {
	for i := 0; i < times; i++ {
		fmt.Print(character)
		time.Sleep(delay)
	}
}

// StringInFile checks if a string exists in a file
func StringInFile(filename string, searchStr string) bool {
	data, err := ioutil.ReadFile(filename)
	if err != nil {
		return false
	}
	return strings.Contains(string(data), searchStr)
}

// findAppWithFastAPI finds an app with FastAPI
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

// extractVariableNameFromLine extracts a variable name from a line
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

// GetAppName gets the app name from code
func GetAppName(code string) (string, bool) {
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

// GetAppNameFromFile gets the app name from a file
func GetAppNameFromFile(filename string) (string, bool) {
	contentBytes, err := os.ReadFile(filename)
	if err != nil {
		return "", false
	}

	contentString := string(contentBytes)

	return GetAppName(contentString)
}
