ARG VERSION

FROM docker.io/kanidm/server:${VERSION} AS kanidm
FROM opensuse/tumbleweed:latest

COPY --from=kanidm /sbin/kanidmd /sbin/

