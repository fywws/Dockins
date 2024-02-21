package table

import (
	"fmt"
	"os"
	"os/exec"
	"strings"

	"github.com/charmbracelet/bubbles/table"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

var baseStyle = lipgloss.NewStyle().
	BorderStyle(lipgloss.NormalBorder()).
	BorderForeground(lipgloss.Color("240"))

type model struct {
	table table.Model
}

func (m model) Init() tea.Cmd { return nil }

const reset = "\033[0m"
const red = "\x1b[31m"
const green = "\x1b[32m"
const bold = "\x1b[1m"
const blue = "\x1b[96m"

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "esc":
			if m.table.Focused() {
				m.table.Blur()
			} else {
				m.table.Focus()
			}
		case "q", "ctrl+c":
			return m, tea.Quit
		}
	}
	updateModel, updateCmd := m.table.Update(msg)
	return model{table: updateModel}, updateCmd
}

func trimString(s string) string {
    start := strings.Index(s, "[") + 1
    end := strings.Index(s, " ")
    return s[start:end]
}

func (m model) View() string {
	return baseStyle.Render(m.table.View()) + "\n"
}

func ListImages() {
	cmdStr := `docker images --format={{.ID}}\t{{.Repository}}\t{{.Tag}}\t{{.Size}}`

	cmdOut, err := exec.Command("cmd", "/c", cmdStr).Output()
	if err != nil {
		fmt.Printf("Error executing command '%s': %v\n", cmdStr, err)
		return
	}

	lines := strings.Split(strings.TrimSpace(string(cmdOut)), "\n")
	imageInfos := make([][]string, len(lines))

	for i, line := range lines {
		parts := strings.Fields(line)
		imageInfos[i] = parts
	}

	rows := make([]table.Row, len(imageInfos))
	for i, row := range imageInfos {
		rows[i] = row
	}

	columns := []table.Column{
		{Title: "Image ID", Width: 20},
		{Title: "Repository", Width: 20},
		{Title: "Tag", Width: 10},
		{Title: "Size", Width: 15},
	}

	t := table.New(
		table.WithColumns(columns),
		table.WithRows(rows),
		table.WithFocused(true),
		table.WithHeight(len(imageInfos)+1),
	)

	s := table.DefaultStyles()
	s.Header = s.Header.
		BorderStyle(lipgloss.RoundedBorder()).
		BorderForeground(lipgloss.Color("#666666")).
		BorderBottom(true).
		Bold(false)
	s.Selected = s.Selected.
		Foreground(lipgloss.Color("229")).
		Background(lipgloss.Color("#575757")).
		Bold(false)
	t.SetStyles(s)

	m := model{t}
	if _, err := tea.NewProgram(m, tea.WithAltScreen()).Run(); err != nil {
		fmt.Println("Error running program:", err)
		os.Exit(1)
	}
}