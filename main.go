package main

import (
	"flag"
	"fmt"
	"github.com/bwmarrin/discordgo"
	"os"
	"os/signal"
	"syscall"
)

var (
	ConfigPath string
)

func init() {
	flag.StringVar(&ConfigPath, "c", "", "ConfigPath")
	flag.Parse()
}

func main() {
	config := LoadConfig(ConfigPath)

	dg, err := discordgo.New("Bot " + config.Token)
	errCheck("Error instantiating Discord client session", err)

	dg.AddHandler(messageCreate)

	err = dg.Open()
	errCheck("Error opening connection", err)

	fmt.Println("Bot is now running–please press Ctrl-C to exit.")
	signalChannel := make(chan os.Signal, 1)
	signal.Notify(signalChannel, syscall.SIGINT, syscall.SIGTERM, os.Interrupt, os.Kill)
	<-signalChannel

	err = dg.Close()
	errCheck("Failed to close discord client properly.", err)
}

func messageCreate(session *discordgo.Session, messageCreate *discordgo.MessageCreate) {
	if messageCreate.Author.ID == session.State.User.ID {
		return
	}

	if messageCreate.Content == "~ping" {
		_, err := session.ChannelMessageSend(messageCreate.ChannelID, "퐁그!")
		errCheck("Failed to send discord message.", err)
	}

	if messageCreate.Content == "~pong" {
		_, err := session.ChannelMessageSend(messageCreate.ChannelID, "핑그!")
		errCheck("Failed to send discord message.", err)
	}
}
