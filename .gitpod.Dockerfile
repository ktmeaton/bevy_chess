FROM gitpod/workspace-full-vnc

USER gitpod

# Install custom tools, runtime, etc. using apt-get

RUN sudo apt-get -q update && \
    sudo apt-get install -yq libasound2-dev && \
    sudo rm -rf /var/lib/apt/lists/*

# More information: https://www.gitpod.io/docs/config-docker/
