# syntax=docker/dockerfile:1.7

FROM gcr.io/distroless/cc-debian12:nonroot

COPY server /server

EXPOSE 5000/udp

ENTRYPOINT ["/server"]
