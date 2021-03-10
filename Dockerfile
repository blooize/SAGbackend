FROM rustlang/rust:nightly

RUN mkdir -p /usr/src/sagbackend
WORKDIR /usr/src/sagbackend

COPY . /usr/src/sagbackend

CMD ["cargo", "build"]