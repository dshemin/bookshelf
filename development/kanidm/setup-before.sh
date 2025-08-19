#!/bin/sh

# Setup Kanidm before start

set -e

if [ -e /data/chain.pem ]; then
	echo "certificates are exists"
	exit 0
fi

ls -la /sbin
kanidmd cert-generate

