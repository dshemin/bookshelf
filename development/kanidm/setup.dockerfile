ARG VERSION

FROM docker.io/kanidm/server:${VERSION} AS server
FROM opensuse/tumbleweed:latest

COPY --from=server /sbin/kanidmd /sbin/

