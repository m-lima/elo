FROM docker.io/node:22.4.1-bookworm
WORKDIR /web

COPY package.json yarn.lock .
RUN yarn

COPY . .
RUN yarn build
