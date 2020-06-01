ARG RUST_VERSION=stable

FROM alpine as builder

ARG MODULE_NAME
ARG MODULE_TAG
ARG MODULE_TARGET

# Building the module
RUN cd /tmp && \
    wget https://github.com/crawlis/${MODULE_NAME}/releases/download/${MODULE_TAG}/${MODULE_NAME}-${MODULE_TAG}-${MODULE_TARGET}-debug && \
    cp /tmp/${MODULE_NAME}-${MODULE_TAG}-${MODULE_TARGET}-debug /tmp/app && \
    chmod +x /tmp/app

FROM clux/muslrust:${RUST_VERSION} as certificates

FROM scratch
COPY --from=builder /tmp/app /
COPY --from=certificates /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
CMD [ "/app" ]