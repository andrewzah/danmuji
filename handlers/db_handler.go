package handlers

import (
	"database/sql"

	. "git.sr.ht/~andrewzah/danmuji/util"
	"github.com/bwmarrin/discordgo"
	_ "github.com/mattn/go-sqlite3"
)

type DbHandler struct {
	Dg     *discordgo.Session
	DbPath string
	db     *sql.DB
}

func (d *DbHandler) Read(session *discordgo.Session, event *discordgo.Ready) {
	d.LoadDb()
	d.MaybePrepareDb()
}

func (d *DbHandler) Msg(session *discordgo.Session, event *discordgo.MessageCreate) {
	if event.Author.ID == session.State.User.ID {
		return
	}

	if event.Content == "~ping" {
		_, err := session.ChannelMessageSend(event.ChannelID, "퐁그!")
		ErrDebug("Failed to send discord message.", err)
	}

	if event.Content == "~pong" {
		_, err := session.ChannelMessageSend(event.ChannelID, "핑그!")
		ErrDebug("Failed to send discord message.", err)
	}

	if event.Content == "~db" {
		_, err := session.ChannelMessageSend(event.ChannelID, "핑핑핑그!")
		ErrDebug("Failed to send discord message.", err)

		d.Insert(event.Author.ID, 1, 2)
	}
}

func (d *DbHandler) LoadDb() {
	db, err := sql.Open("sqlite3", d.DbPath)
	ErrPanic("Unable to load database file.", err)
	d.db = db
}

func (d *DbHandler) MaybePrepareDb() {
	prepareText := `
	  CREATE TABLE IF NOT EXISTS users (
	    id INTEGER PRIMARY KEY,
	    discordUserId TEXT,
	    koreanCount INTEGER,
	    nonKoreanCount INTEGER
	  )
	`

	stmt, err := d.db.Prepare(prepareText)
	ErrPanic("Unable to prepare database.", err)

	_, err = stmt.Exec()
	ErrPanic("Unable to exec preparation sql.", err)
}

func (d *DbHandler) Insert(dUId string, kCount int, nonKCount int) {
	prepareText := `
	  INSERT INTO users
	    (discordUserId, koreanCount, nonKoreanCount)
	  VALUES
	    (?,?,?)
	`

	stmt, err := d.db.Prepare(prepareText)
	ErrDebug("Unable to prepare insert query.", err)

	_, err = stmt.Exec(dUId, kCount, nonKCount)
	ErrDebug("Unable to insert to database.", err)
}
