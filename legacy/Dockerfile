# This must be run with the Docker context set to the root folder of the repository
# (the one with the yarn.lock file)

FROM node:16-alpine

# App directory
WORKDIR /usr/src

ENV NODE_ENV production

COPY ./package.json ./package.json
RUN yarn --network-timeout 1000000 install

COPY . .
RUN yarn build

EXPOSE ${BACKEND_PORT}
CMD [ "yarn", "prod" ]
