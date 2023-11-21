FROM rustlang/rust:nightly
RUN rustup default nightly
COPY . .
ENV TUX_DIR=${TUX_DIR}
ENV TUX_EDITOR=${TUX_EDITOR}
EXPOSE 3000
EXPOSE 433
CMD ["cargo", "run","--release"]

