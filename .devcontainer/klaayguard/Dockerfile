# ---- Dev Container ----
FROM docker.io/library/rust:1.87-bookworm AS dev

# Install Node.js 22.x and dependencies
RUN apt-get update && \
    apt-get install -y curl build-essential pkg-config libssl-dev && \
    curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get install -y nodejs && \
    node --version && npm --version && \
    npm install -g yarn

# tauri deps
RUN apt install -y libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    xdg-utils

RUN apt install -y cowsay

# Set workdir for bind mount
WORKDIR /app

COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT [ "/entrypoint.sh" ]

COPY command.sh /command.sh
CMD [ "/command.sh"]
