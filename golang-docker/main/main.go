package main

import (
	"fmt"
	dt "main/docker_templates"
	"os"

	"github.com/spf13/cobra"
)

const colorReset = "\033[0m"

func main() {
	var rootCmd = &cobra.Command{Use: "dockn"}

	var pingCmd = &cobra.Command{
		Use: "ping",
		Run: func(cmd *cobra.Command, args []string) {
            fmt.Println( "\x1b[32m" + "PONG" + colorReset)
        },
	}

	var initCmd = &cobra.Command{
		Use:   "init",
		Short: "Initialize Dockerfile",
		Args:  cobra.ExactValidArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			arg := args[0]
			noScript := cmd.Flag("no-script").Changed

			switch arg {
			case "go":
				dt.GO_Write(noScript)
			case "rust":
				dt.RUST_Write(noScript)
			case "node":
				dt.NODEJS_Write(noScript)	
			default:
				fmt.Println( "\x1b[31m"+ "\x1b[1m" + " → ERROR : incorrect language provided" + colorReset)
				os.Remove("Dockerfile")
			}

		},
	}
	
	initCmd.Flags().Bool("no-script", true, "No script creating with |init| command")
	




	
	var helpCmd = &cobra.Command{
		Use:   "help",
        Short: "Show help",
		Long: "Show information of all commands and arguments",
        Args:  cobra.ExactValidArgs(0),
        Run: func(cmd *cobra.Command, args []string) {
            fmt.Println(`List of |init| arguments : 
	-> go - GoLang
	-> java - Java
	-> rust - Rust 
	-> nodejs - Yarn and npm node-js projects`)
        },
	}
	
	rootCmd.AddCommand(initCmd, helpCmd, pingCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

