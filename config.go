package main

import (
	"encoding/json"
	"os"
)

type Config struct {
	Token string
}

func LoadConfig(file string) Config {
	var config Config
	configFile, err := os.Open(file)
	errCheck("Failed to load Config file.", err)

	jsonParser := json.NewDecoder(configFile)
	err = jsonParser.Decode(&config)
	errCheck("Failed to parse Config.json", err)

	return config
}
