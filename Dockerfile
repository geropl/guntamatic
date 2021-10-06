FROM alpine:3.13

RUN apk add bash curl

RUN mkdir -p /app
WORKDIR /app
COPY guntamatic /app/guntamatic
ENTRYPOINT [ "/app/guntamatic" ]