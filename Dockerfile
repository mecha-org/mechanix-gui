FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive \
    CARGO_HOME=/opt/cargo \
    RUSTUP_HOME=/opt/rustup \
    PATH=/opt/cargo/bin:/opt/rustup/bin:/root/.local/bin:$PATH

# Base deps
RUN apt update && apt install -y \
    curl \
    git \
    openssh-client \
    fakeroot \
    jq \
    libdbus-1-dev \
    libpulse-dev \
    libpam0g-dev \
    libclang-dev \
    librust-gdk-pixbuf-sys-dev \
    python3 \
    python3-pip \
    pipx \
    ca-certificates \
    gnupg \
    && rm -rf /var/lib/apt/lists/*

# Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable

# Cargo tools
RUN cargo install cargo-deb cargo-generate-rpm

# Nushell
RUN curl -fsSL https://apt.fury.io/nushell/gpg.key | gpg --dearmor \
      -o /etc/apt/trusted.gpg.d/fury-nushell.gpg \
    && echo "deb https://apt.fury.io/nushell/ /" \
      > /etc/apt/sources.list.d/nushell.list \
    && apt update && apt install -y nushell \
    && rm -rf /var/lib/apt/lists/*

# Pulp CLI
RUN pipx install pulp-cli[pygments] \
    && pipx inject pulp-cli pulp-cli-deb pulp_rpm

WORKDIR /work
