package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"main/docker_templates"
	"main/libs"
	"main/libs/input"
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
			config, _ := cmd.Flags().GetString("config")

			var choice string
			var makeS bool = false
			var portN int = 8080
			var db string = "none"

			if config == "" {

				choice = list.InitList("Pick language", []string{"python", "go", "rust", "nodejs"})

				// INITIALIZING A MAKESCRIPT LSIT
				makeScript := list.InitList("Create a launch script?", []string{"yes", "no"})
				if makeScript == "yes" {
					makeS = true
				}

				// INITIALIZING A PORT INPUT
				input := input.InitInput("PORT", "Provide a port for the container")
				port, err := strconv.Atoi(input)
				if err != nil {
					fmt.Println("\x1b[31m" + "\x1b[1m" + "× ERROR: incorrect port provided" + colorReset)
					os.Exit(1)
				}
				portN = port

				// INITIALIZING A DB INPUT
				db = list.InitList("Choose a database", []string{"none", "mysql", "postgres", "mongodb"})
				if db == "none" {
					db = ""
				}
			} else {
				c, _ := libs.GetProperty("language", config)
				choice = strings.Trim(c, " ")

				mks, _ := libs.GetProperty("make-script", config)
				makeString := strings.Trim(mks, " ")
				if makeString == "true" {
					makeS = true
				}

				p, _ := libs.GetProperty("port", config)
				portNum := strings.Trim(p, " ")
				portN, _ = strconv.Atoi(portNum)

				d, _ := libs.GetProperty("db", config)
				db = strings.Trim(d, " ")
			}

			switch choice {
			case "python":
				docker_templates.PY_Write(makeS, portN, db)
			case "go":
				docker_templates.GO_Write(makeS, portN, db)
			case "rust":
				docker_templates.RUST_Write(makeS, portN, db)
			case "nodejs":
				docker_templates.NODEJS_Write(makeS, portN, db)
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
	initCmd.PersistentFlags().String("config", "", "Path to config file")

	rootCmd.AddCommand(initCmd, imagesCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
