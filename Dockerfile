FROM rustlang/rust:nightly

RUN mkdir -p /usr/src/sagbackend
WORKDIR /usr/src/sagbackend

COPY . /usr/src/sagbackend
RUN cargo build

EXPOSE 3000

CMD [ "cargo", "run" ]