package main

// go build -trimpath -o dockn.exe -ldflags "-X main.version=1.0.0 -X main.buildDate=$(date -u +'%Y-%m-%dT%H:%M:%SZ')" .\main\main.go
import (
	"fmt"
	dt "main/docker_templates"
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

	var initCmd = &cobra.Command{
		Use:   "init",
		Short: "Initialize Dockerfile",
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
				dt.RUST_Write(noScript, port)
			case "node":
				dt.NODEJS_Write(noScript, port)	
			case "cpp":
				dt.CPP_Write(noScript, port)
			case "py":
				dt.PY_Write(noScript, port)	
			default:
				fmt.Println( "\x1b[31m"+ "\x1b[1m" + " × ERROR : incorrect language provided" + colorReset)
				os.Remove("Dockerfile")
			}

		},
	}
	
	initCmd.Flags().Bool("no-script", true, "No script creating with |init| command")
	initCmd.Flags().String("port", "3000", "Specify port for Docker to run")
	initCmd.Flags().String("db", "", "Specify port for Docker to run")



	

	
	rootCmd.AddCommand(initCmd, pingCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

