#!/bin/sh

# Setup Kanidm before start

set -e

if [ -e /data/chain.pem ]; then
	echo "Certificates are exists, skip"
	exit 0
fi

echo "Generate certificates"
kanidmd cert-generate

