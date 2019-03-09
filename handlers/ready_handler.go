package handlers

import (
	. "git.sr.ht/~andrewzah/danmuji/util"
	"github.com/bwmarrin/discordgo"
)

type ReadyHandler struct {
	Dg     *discordgo.Session
	Status string
}

func (r *ReadyHandler) Read(session *discordgo.Session, event *discordgo.Ready) {
	err := session.UpdateStatus(0, r.Status)
	ErrDebug("Failed to update status.", err)
}
