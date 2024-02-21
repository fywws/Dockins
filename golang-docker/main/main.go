package main

// go build -trimpath -o dockn.exe -ldflags "-X main.version=1.0.0 -X main.buildDate=$(date -u +'%Y-%m-%dT%H:%M:%SZ')" .\main\main.go
import (
	"fmt"
	dt "main/docker_templates"
	input "main/libs/input"
	inputs "main/libs/inputs"
	list "main/libs/list"
	table "main/libs/table"

	"os"
	"strconv"

	"github.com/spf13/cobra"
)

const colorReset = "\033[0m"

func main() {
	var rootCmd = &cobra.Command{Use: "dockn"}

	var initCmd = &cobra.Command{
		Use: "init",
		Short: "Generating Dockerfile",
		Long: "Generating Dockerfile graphically",
		Run: func (cmd *cobra.Command, args []string){
			choice := list.InitList("Pick language", []string{"python","go", "rust", "nodejs"})

			makeScript := list.InitList("Do you want to create a lauch script?", []string{"Yes", "No"})
			var makeSbool bool;
			
			switch makeScript {
				case"Yes":
					makeSbool = false
				case"No":
					makeSbool = true
				default:
					fmt.Println( "\x1b[31m"+ "\x1b[1m" + " × ERROR : No choise provided" + colorReset)
					os.Exit(0)
            }			
	
			input := input.InitInput("PORT", "Choose port to app must connect")


			db := list.InitList("Choose database", []string{"none", "postgres"})
					
			if db == "none" {
				db=""
			}

			port, err := strconv.Atoi(input)	
			if err != nil {
                fmt.Println( "\x1b[31m" + "\x1b[1m" + "× ERROR: incorrect port provided" + colorReset)
				os.Exit(0)

			}

			switch choice {
			case "python":
				dt.PY_Write(makeSbool, port , db)
			case "go":
				dt.GO_Write(makeSbool, port , db)
			case "rust":
				dt.RUST_Write(makeSbool, port , db)	
			case "nodejs":
				dt.NODEJS_Write(makeSbool, port , db)	
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

	var inputsCmd = &cobra.Command{
		Use:   "inputs",
        Short: "List Docker inputs",
        Long:  "List Docker inputs",
        Args:  cobra.ExactValidArgs(0),
        Run: func(cmd *cobra.Command, args []string) {
            inputs.InitInputs()
        },
	}
	
	initCmd.Flags().Bool("no-script", true, "No script creating with |init| command")
	initCmd.Flags().String("port", "3000", "Specify port for Docker to run")
	initCmd.Flags().String("db", "", "Specify port for Docker to run")
	

	
	rootCmd.AddCommand(initCmd, imagesCmd, inputsCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

