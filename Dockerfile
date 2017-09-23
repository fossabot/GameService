FROM base/archlinux
RUN mkdir -p /app
WORKDIR /app
ARG rocket_port=8000
ARG rocket_log=critical
ARG game_service_branch=master
RUN pacman -Syy archlinux-keyring --noconfirm
RUN pacman -Syyu --noconfirm
RUN pacman -S rustup postgresql make clang git --noconfirm
RUN rustup default nightly
RUN git clone -b $game_service_branch https://github.com/MikiBot/GameService .
ENV DATABASE_URL='postgres://postgres@localhost/GameService'
ENV TEST_DATABASE_URL='postgres://postgres@localhost/GameServiceTest'
ENV ROCKET_ADRESS=0.0.0.0
ENV ROCKET_PORT=$rocket_port
ENV ROCKET_LOG=$rocket_log

RUN cargo build --release;

EXPOSE $rocket_port
CMD = ["/app/target/release/game-service"]
