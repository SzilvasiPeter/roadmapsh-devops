FROM node:alpine

# TODO: Add ARG to inject value from CI
ENV SECRET_MESSAGE="This is top secret!"
ENV USERNAME=dockeruser
ENV PASSWORD=dockerpassword

WORKDIR /usr/src/app

# Install dependencies first (better layer caching)
COPY package*.json ./
RUN npm ci --only=production

# Copy application code
COPY server.js login.html ./

EXPOSE 3000

CMD ["node", "server.js"]
