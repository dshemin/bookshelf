#!/bin/sh

set -e

GROUP_NAME="bookshelf_users"

main() {
	echo "Create users"
	create_users

	echo "Create OAuth"
	create_oauth
}

create_oauth() {
	NAME="bookshelf"
	REDIRECT_URL="https://127.0.0.1.nip.io/oauth2/handler"

	echo "Create client"
	kanidm system oauth2 create $NAME "Bookshelf" https://idm.127.0.0.1.nip.io --name idm_admin

	echo "Add redirect url"
	kanidm system oauth2 add-redirect-url $NAME $REDIRECT_URL --name idm_admin

	echo "Update scope"
	kanidm system oauth2 update-scope-map $NAME $GROUP_NAME email profile openid --name idm_admin

	kanidm system oauth2 get $NAME --name idm_admin
	SECRET=$(kanidm system oauth2 show-basic-secret $NAME --name idm_admin 2>/dev/null)

	echo "Write .env.local"
	cat << EOF > .env.local
BOOKSHELF_AUTH_URL=https://idm.127.0.0.1.nip.io:8443/ui/oauth2
BOOKSHELF_TOKEN_URL=https://idm.127.0.0.1.nip.io:8443/oauth2/token
BOOKSHELF_CLIENT_ID=$NAME
BOOKSHELF_CLIENT_SECRET=$SECRET
BOOKSHELF_REDIRECT_URL=$REDIRECT_URL
EOF
}

create_users() {
	kanidm group create $GROUP_NAME --name idm_admin

	create_user "demo"
}

create_user() {
	# Manually run `kanidm person credential update` for each user.
	name=$1
	kanidm person create $name "Demonstration User" --name idm_admin
	kanidm group add-members $GROUP_NAME $name
}

main
