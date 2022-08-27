FROM gitpod/workspace-full

RUN sudo apt-get -q update && \
    sudo apt-get install -yq libseccomp-dev && \
    sudo rm -rf /var/lib/apt/lists/*