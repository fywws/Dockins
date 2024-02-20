package main

import (
	"fmt"
	dt "main/docker_templates"
	"os"
	"time"

	"github.com/spf13/cobra"
)

func main() {
	var rootCmd = &cobra.Command{Use: "dockn"}

	var initCmd = &cobra.Command{
		Use:   "init",
		Short: "Initialize Dockerfile",
		Args:  cobra.ExactValidArgs(1), 
		Run: func(cmd *cobra.Command, args []string) {
			lang := args[0]
			
			os.Create("Dockerfile")
			fmt.Print("Initializing... "+ lang + "\t")
			writePretty()
			
			switch lang {
			case "go":
				dt.GO_Write("Dockerfile")
			case "rust":
				dt.RUST_Write("Dockerfile")
			}


		},
	}

	rootCmd.AddCommand(initCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

const colorReset = "\033[0m"
const colorRed = "\033[45m"

func writePretty() {
	fmt.Print("[")
	repeatWithDelay(colorRed+"#"+colorReset, 2200, 29)
	fmt.Println("]")
}
func repeatWithDelay(character string, delay time.Duration, times int) {
	for i := 0; i < times; i++ {
		fmt.Print(character)
		time.Sleep(delay)
	}
}