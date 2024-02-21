package main

// go build -trimpath -o dockn.exe -ldflags "-X main.version=1.0.0 -X main.buildDate=$(date -u +'%Y-%m-%dT%H:%M:%SZ')" .\main\main.go
import (
	"fmt"
	dt "main/docker_templates"
	input "main/libs/input"
	list "main/libs/list"
	"os"
	"strconv"

	"github.com/spf13/cobra"
)

const colorReset = "\033[0m"

func main() {
	var rootCmd = &cobra.Command{Use: "dockn"}

	var pingCmd = &cobra.Command{
		Use: "ping",
		Run: func(cmd *cobra.Command, args []string) {
            fmt.Println( "\x1b[32m" + "\x1b[1m" + "PONG" + colorReset)
        },
	}

	var generateCmd = &cobra.Command{
		Use: "generate",
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
            }			
	
			input := input.InitInput("PORT", "Choose port to app must connect")


			db := list.InitList("Choose database", []string{"none", "postgres"})
					
			if db == "none" {
				db=""
			}

			port, err := strconv.Atoi(input)
			if err != nil {
                fmt.Println( "\x1b[31m" + "\x1b[1m" + "× ERROR: incorrect port provided" + colorReset)
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

	var initCmd = &cobra.Command{
		Use:   "init",
		Short: "Initialize Dockerfile [ DEPERECATED: use generate instead ]",
		Long: "Initialize Dockerfile with options [ DEPERECATED: use generate instead ]",
		Args: cobra.ExactValidArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			arg := args[0]
			noScript := cmd.Flag("no-script").Changed
			portStr := cmd.Flag("port").Value.String()
			dbStr := cmd.Flag("db").Value.String()

			port, err := strconv.Atoi(portStr)
            if err != nil {
                fmt.Println( "\x1b[31m" + "\x1b[1m" + "× ERROR: incorrect port provided" + colorReset)
                return
            }

			var db string;

			switch dbStr {
			case "":
				break

			case "postgres":
				db = "postgres"
			case "mysql":
				db = "mysql"
			case "sqlite":
				db = "sqlite"

			default:
				fmt.Println( "\x1b[31m" + "\x1b[1m" + " × ERROR: provided database not found" + colorReset)
                return
			}


			switch arg {
			case "go":
				dt.GO_Write(noScript, port, db)
			case "rust":
				dt.RUST_Write(noScript, port, db)
			case "node":
				dt.NODEJS_Write(noScript, port, db)	
			case "cpp":
				dt.CPP_Write(noScript, port, db)
			case "py":
				dt.PY_Write(noScript, port, dbStr)	
			default:
				fmt.Println( "\x1b[31m"+ "\x1b[1m" + " × ERROR : incorrect language provided" + colorReset)
				os.Remove("Dockerfile")
			}

		},
	}
	
	initCmd.Flags().Bool("no-script", true, "No script creating with |init| command")
	initCmd.Flags().String("port", "3000", "Specify port for Docker to run")
	initCmd.Flags().String("db", "", "Specify port for Docker to run")
	
	generateCmd.Flags().Bool("no-script", true, "No script creating with |init| command")
	generateCmd.Flags().String("port", "3000", "Specify port for Docker to run")
	generateCmd.Flags().String("db", "", "Specify port for Docker to run")

	
	rootCmd.AddCommand(initCmd, pingCmd, generateCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

