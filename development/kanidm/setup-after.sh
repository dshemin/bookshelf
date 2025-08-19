#!/bin/sh

# Setup Kanidm after startup.

set -e

PASS_FILE='/data/admin.pass'

main() {
	recover_pass
}

recover_pass() {
	# Make sure we didn't run twice.
	if [ -e $PASS_FILE ]; then
		echo "Admin password already recovered"
		return
	fi

	kanidmd recover-account admin 2>/dev/null | \
		grep new_password | \
		cut -d ':' -f 3 | \
		tr -d ' "' > $PASS_FILE
}

main
