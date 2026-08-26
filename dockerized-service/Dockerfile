FROM node:alpine

# TODO: Use BuildKit secret mounts (--mount=type=secret) instead of ARG or ENV
ARG SECRET_MESSAGE="This is top secret!"
ARG USERNAME=dockeruser
ARG PASSWORD=dockerpassword

ENV SECRET_MESSAGE=$SECRET_MESSAGE
ENV USERNAME=$USERNAME
ENV PASSWORD=$PASSWORD

WORKDIR /usr/src/app

# Install dependencies first (better layer caching)
COPY package*.json ./
RUN npm ci --only=production
COPY server.js login.html ./

EXPOSE 3000
CMD ["node", "server.js"]
