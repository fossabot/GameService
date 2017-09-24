FROM base/archlinux
RUN mkdir -p /app
ARG rocket_port=8000
ARG rocket_log=critical
ARG database_url=postgres://postgres@localhost/GameService
WORKDIR /app
ARG game_service_branch=master
RUN pacman -Syy archlinux-keyring --noconfirm
RUN pacman -Syyu --noconfirm
RUN pacman -S rustup postgresql make clang git --noconfirm
RUN rustup default nightly
RUN git clone -b $game_service_branch https://github.com/MikiBot/GameService .
RUN cargo build --release;
ENV DATABASE_URL=$database_url
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=$rocket_port
ENV ROCKET_LOG=$rocket_log
EXPOSE $rocket_port
COPY target/release/game-service /
RUN ls /
RUN rm -rf /app /bin /home /lib /lib64  /root /run /sbin /srv
ENTRYPOINT ["/game-service"]
