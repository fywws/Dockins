package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

func main() {
	var rootCmd = &cobra.Command{Use: "dockn"}

	var initCmd = &cobra.Command{
		Use:   "init",
		Short: "Initialize Dockerfile",
		Run: func(cmd *cobra.Command, args []string) {
			os.Create("Dockerfile")
		},
	}

	rootCmd.AddCommand(initCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
