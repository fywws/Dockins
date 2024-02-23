package libs

import (
	"bufio"
	"errors"
	"os"
	"strings"
)

func GetProperty(key string, filename string) (string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return "", err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, key+"=") {
			return strings.TrimPrefix(line, key+"="), nil
		}
	}

	err = scanner.Err()
	if err != nil {
		return "", errors.New("property not found")
	}

	return "", nil
}