package main

import (
	"context"
	"encoding/json"
	"time"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/sirupsen/logrus"
)

func handler(ctx context.Context, kinesisEvent events.KinesisEvent) error {
	for _, record := range kinesisEvent.Records {
		kinesisRecord := record.Kinesis
		dataBytes := kinesisRecord.Data
		event := &ModelOne{}
		err := json.Unmarshal(dataBytes, event)
		if err != nil {
			logrus.WithFields(logrus.Fields{
				"err": err,
			}).Error("Error occurred unmarshalling")
			continue
		}
		event.ReadTime = time.Now()
		logrus.WithFields(logrus.Fields{
			"event":      string(dataBytes),
			"marshalled": event,
		}).Info("Printing out the event")
	}

	return nil
}

func init() {
	logrus.SetFormatter(&logrus.JSONFormatter{})
	logrus.SetLevel(logrus.DebugLevel)
}

func main() {
	lambda.Start(handler)
}
