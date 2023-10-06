FROM rust:1.69.0 AS build-env

ARG BUILD_DATE
ARG VCS_REF
LABEL maintainer="Laurent <laurent@vromman.org>" \
    org.opencontainers.image.title="Brevo to Discord webhook" \
    org.opencontainers.image.description="A simple webhook listener that receives Brevo event and publish them to a discord channel" \
    org.opencontainers.image.authors="Laurent <laurent@vromman.org>" \
    org.opencontainers.image.vendor="Laurent Vromman" \
    org.opencontainers.image.documentation="https://github.com/leirn/brevo_discord_webhook/README.md" \
    org.opencontainers.image.licenses="MIT" \
    org.opencontainers.image.version="0.1.0" \
    org.opencontainers.image.url="https://github.com/leirn/brevo_discord_webhook/" \
    org.opencontainers.image.source="https://github.com/leirn/brevo_discord_webhook/" \
    org.opencontainers.image.revision=$VCS_REF \
    org.opencontainers.image.created=$BUILD_DATE

WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
ENV HOST=0.0.0.0

ENV PORT=8080

ARG AUTHORIZED_IP_RANGES
ENV AUTHORIZED_IP_RANGES=${AUTHORIZED_IP_RANGES}

ARG DISCORD_WEBHOOK_TOKEN
ENV DISCORD_WEBHOOK_TOKEN=${DISCORD_WEBHOOK_TOKEN}

ARG RUST_LOG="warn"
ENV RUST_LOG=${RUST_LOG}

ARG RUST_BACKTRACE="0"
ENV RUST_BACKTRACE=${RUST_BACKTRACE}

EXPOSE 8080

COPY --from=build-env /app/target/release/brevo_discord_webhook /
CMD ["./brevo_discord_webhook"]