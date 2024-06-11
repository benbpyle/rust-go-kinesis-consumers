package main

import (
	"time"
)

type ModelOne struct {
	WriteTime time.Time `json:"writeTime"`
	ReadTime  time.Time `json:"readTime"`
	Id        string    `json:"id"`
	Name      string    `json:"name"`
	Locations []string  `json:"locations"`
}
