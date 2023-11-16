FROM gitpod/workspace-full

USER gitpod

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN rustup default stable