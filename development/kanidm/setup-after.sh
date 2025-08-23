#!/bin/sh

# Setup Kanidm after startup.

set -e

ADMIN_PASS_FILE='/data/admin.pass'
IDM_ADMIN_PASS_FILE='/data/idm-admin.pass'

main() {
	recover admin $ADMIN_PASS_FILE
	recover idm_admin $IDM_ADMIN_PASS_FILE
}

recover() {
	user="$1"
	path="$2"

	if [ -e $path ]; then
		echo "Password for $user already recovered"
		return
	fi

	echo "Recover password for $user"
	kanidmd recover-account $user 2>/dev/null | \
		grep new_password | \
		cut -d ':' -f 3 | \
		tr -d ' "' > $path
}

main
