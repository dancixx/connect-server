FROM rust:latest as build

WORKDIR /usr/src/connect-backend
COPY . .

RUN cargo build --release

FROM ubuntu:22.04

RUN apt-get update -y \
    && apt-get install -y libpq-dev curl \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && curl -sSf https://install.surrealdb.com | sh 
    
COPY --from=build /usr/src/connect-backend/target/release/connect-backend /usr/local/bin/connect-backend
COPY --from=build /usr/src/connect-backend/.env /usr/local/bin/.env
COPY --from=build /usr/src/connect-backend/migrations /usr/local/bin/migrations
COPY --from=build /usr/src/connect-backend/schemas /usr/local/bin/schemas
COPY --from=build /usr/src/connect-backend/events /usr/local/bin/events

WORKDIR /usr/local/bin

# Create a custom entrypoint script
RUN echo '#!/bin/bash\n\
surreal start --log trace --user root --pass root --bind 127.0.0.1:8000 file:test.db &\n\
./connect-backend\n' > entrypoint.sh \
    && chmod +x entrypoint.sh

CMD ["./entrypoint.sh"]
