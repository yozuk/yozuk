FROM gitpod/workspace-full

RUN sudo apt-get -q update && \
    sudo apt-get install -yq libpython3.6 rust-lldb libseccomp-dev && \
    sudo rm -rf /var/lib/apt/lists/*

RUN curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin
RUN curl https://get.wasmer.io -sSfL | sh
RUN cargo install just taplo-cli wasm-pack

ENV RUST_LLDB=/usr/bin/lldb-8