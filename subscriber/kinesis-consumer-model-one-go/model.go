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

// func (m *ModelOne) UnmarshalJSON(data []byte) error {
// 	var v ModelOne
// 	if err := json.Unmarshal(data, &v); err != nil {
// 		return err
// 	}
//
// 	m.Id = v.Id
// 	m.Name = v.Name
// 	m.Locations = v.Locations
// 	m.WriteTime = v.WriteTime
// 	m.ReadTime = time.Now()
// 	return nil
// }
