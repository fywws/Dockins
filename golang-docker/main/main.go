package main

import (
	"fmt"
	dt "main/docker_templates"
	"os"

	"github.com/spf13/cobra"
)

func main() {
	var rootCmd = &cobra.Command{Use: "dockn"}

	var initCmd = &cobra.Command{
		Use:   "init",
		Short: "Initialize Dockerfile",
		Run: func(cmd *cobra.Command, args []string) {
			lang, _ := cmd.Flags().GetString("language")
			
			fmt.Println("Initializing... " + lang)
			
			os.Create("Dockerfile")

			switch lang {
			case "go":
				dt.GO_Write("Dockerfile")
			case "rust":
				dt.RUST_Write("Dockerfile")
			}

			defer fmt.Println("NOT FOUND: " + lang)
			
		},
	}
	
	initCmd.Flags().StringP("language", "l", "", "Programing language of Dockerfile")
	initCmd.MarkFlagRequired("language")

	rootCmd.AddCommand(initCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}