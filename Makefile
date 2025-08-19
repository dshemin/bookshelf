.PHONY:
help:		## Show this help
	@sed -ne '/@sed/!s/## //p' $(MAKEFILE_LIST)

.PHONY: cleanup
cleanup: 	## Remove all generated development files.
	rm -rf ./development/kanidm/data/*.pem
	rm -rf ./development/kanidm/data/kanidm*
	rm ./development/kanidm/data/*.pass

