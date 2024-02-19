package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

func main() {
	var rootCmd = &cobra.Command{Use: "app"}

	var privetCmd = &cobra.Command{
		Use:   "privet",
		Short: "Say hello",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println("Привет!")
		},
	}

	var pokaCmd = &cobra.Command{
		Use:   "poka",
		Short: "Say goodbye",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println("Пока!")
		},
	}

	rootCmd.AddCommand(privetCmd, pokaCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
