package main

import (
	"encoding/json"
	"os"

	. "git.sr.ht/~andrewzah/danmuji/util"
)

type Config struct {
	Token  string
	Status string
	DbPath string
}

func LoadConfig(file string) Config {
	var config Config

	configFile, err := os.Open(file)
	ErrPanic("Failed to load Config file.", err)

	jsonParser := json.NewDecoder(configFile)
	err = jsonParser.Decode(&config)
	ErrPanic("Failed to parse Config.json", err)

	return config
}
