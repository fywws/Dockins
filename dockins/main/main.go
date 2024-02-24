package main

import (
	"fmt"
	"os"
	"strconv"

	"main/docker_templates"
	"main/libs/list"
	"main/libs/table"

	"github.com/spf13/cobra"
)

const colorReset = "\033[0m"

// main function
func main() {
	var rootCmd = &cobra.Command{Use: "dockn"}

	var initCmd = &cobra.Command{
		Use:   "init",
		Short: "Generating Dockerfile",
		Long:  "Generating Dockerfile graphically",
		Run: func(cmd *cobra.Command, args []string) {
			choice := list.InitList("Pick language", []string{"python", "go", "rust", "nodejs"})

			makeScript, _ := cmd.Flags().GetBool("no-script")
			input, _ := cmd.Flags().GetString("port")
			db, _ := cmd.Flags().GetString("db")

			if db == "none" {
				db = ""
			}

			port, err := strconv.Atoi(input)
			if err != nil {
				fmt.Println("\x1b[31m" + "\x1b[1m" + "× ERROR: incorrect port provided" + colorReset)
				os.Exit(1)
			}

			switch choice {
			case "python":
				docker_templates.PY_Write(makeScript, port, db)
			case "go":
				docker_templates.GO_Write(makeScript, port, db)
			case "rust":
				docker_templates.RUST_Write(makeScript, port, db)
			case "nodejs":
				docker_templates.NODEJS_Write(makeScript, port, db)
			default:
				fmt.Println("\x1b[31m" + "\x1b[1m" + " × ERROR : Invalid language choice" + colorReset)
				os.Exit(1)
			}
		},
	}

	var imagesCmd = &cobra.Command{
		Use:   "images",
		Short: "List Docker images",
		Long:  "List Docker images",
		Args:  cobra.ExactValidArgs(0),
		Run: func(cmd *cobra.Command, args []string) {
			table.ListImages()
		},
	}

	// Flags for init command
	initCmd.Flags().Bool("no-script", false, "No script creating with |init| command")
	initCmd.Flags().String("port", "3000", "Specify port for Docker to run")
	initCmd.Flags().String("db", "", "Specify database for Docker to run")

	rootCmd.AddCommand(initCmd, imagesCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
