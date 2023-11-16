FROM gitpod/workspace-rust:2023-11-16-11-19-36

USER gitpod

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN rustup default stable