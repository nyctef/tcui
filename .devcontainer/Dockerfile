FROM rust:1

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
    && apt-get -y install --no-install-recommends apt-utils dialog 2>&1 \
    # Verify git, needed tools installed
    && apt-get -y install git iproute2 procps lsb-release \
    # Install lldb, vadimcn.vscode-lldb VSCode extension dependencies
    && apt-get install -y lldb python3-minimal libpython3.7 \
    # Install Rust components
    && rustup update \
    && rustup component add rls rust-analysis rust-src rustfmt clippy \
    # Create a non-root user to use - this is referenced in devcontainer.json
    && groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    # Add sudo support for the non-root user
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME\
    && chmod 0440 /etc/sudoers.d/$USERNAME \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

ENV DEBIAN_FRONTEND=dialog