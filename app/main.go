package main

import (
	"flag"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"github.com/bwmarrin/discordgo"

	h "git.sr.ht/~andrewzah/danmuji/handlers"
	. "git.sr.ht/~andrewzah/danmuji/util"
)

var (
	ConfigPath string
	DbPath     string
)

func init() {
	flag.StringVar(&ConfigPath, "c", "./config.json", "ConfigPath")
	flag.StringVar(&DbPath, "d", "./danmuji.db", "DatabasePath")
	flag.Parse()
}

func main() {
	config := LoadConfig(ConfigPath)

	dg, err := discordgo.New("Bot " + config.Token)
	ErrPanic("Error instantiating Discord client session", err)

	// Ready– Set status, etc.
	readyHandler := h.ReadyHandler{Dg: dg, Status: config.Status}
	dg.AddHandler(readyHandler.Read)

	// Database– Initialize and load db
	dbHandler := h.DbHandler{Dg: dg, DbPath: config.DbPath}
	dg.AddHandler(dbHandler.Read)

	// Temporary message handler
	// TODO: break out into command/msg handler
	dg.AddHandler(dbHandler.Msg)

	err = dg.Open()
	ErrPanic("Error opening connection", err)

	fmt.Println("Bot is now running–please press Ctrl-C to exit.")
	signalChannel := make(chan os.Signal, 1)
	signal.Notify(signalChannel, syscall.SIGINT, syscall.SIGTERM, os.Interrupt, os.Kill)
	<-signalChannel

	err = dg.Close()
	ErrPanic("Failed to close discord client properly.", err)
}
