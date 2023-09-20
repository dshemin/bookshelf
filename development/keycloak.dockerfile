# syntax=docker/dockerfile:1
ARG keycloak_version=21.1.1

FROM golang:1.20 as builder

COPY <<EOF /root/go.mod
module pinger

go 1.21.0
EOF

COPY <<EOF /root/main.go
package main

import (
	"errors"
	"fmt"
	"log"
	"net/http"
	"os"
)

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}

func run() error {
	res, err := http.Get(os.Args[1])
	if err != nil {
		return fmt.Errorf("get: %w", err)
	}

	if res.StatusCode != 200 {
		return errors.New("not 200")
	}
	return nil
}
EOF

RUN go build -o /root/pinger /root/main.go

FROM quay.io/keycloak/keycloak:${keycloak_version}

COPY --from=builder --chown=1000:1000 /root/pinger /usr/local/bin/pinger