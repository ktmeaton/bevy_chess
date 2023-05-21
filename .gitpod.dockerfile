FROM gitpod/workspace-full-vnc

RUN sudo apt-get update \
  && sudo apt-get install -y \
    libasound2-dev
