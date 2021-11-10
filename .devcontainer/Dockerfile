# [Choice] Debian version: bullseye, buster, stretch
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends build-essential make curl bash zsh fish && \
    apt-get clean -y && rm -rf /var/lib/apt/lists/*

# docker-cli
RUN apt-get update && \
    apt-get -y install --no-install-recommends apt-transport-https ca-certificates curl gnupg lsb-release && \
    curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null && \
    apt-get update && apt-get -y install docker-ce-cli && \
    apt-get clean -y && rm -rf /var/lib/apt/lists/*

# install rustfmt
RUN rustup component add rustfmt

# install clippy
RUN rustup component add clippy

# install rust tools
RUN cargo install cargo-edit cargo-update

# install docker
ARG DOCKER_VERSION=20.10.7
RUN curl -Lo docker.tgz "https://download.docker.com/linux/static/stable/x86_64/docker-${DOCKER_VERSION}.tgz" && \
    tar --extract --file docker.tgz --strip-components 1 --directory /usr/local/bin/ && rm docker.tgz

# install kind
ARG KIND_VERSION=0.11.1
RUN curl -Lo kind https://kind.sigs.k8s.io/dl/v${KIND_VERSION}/kind-linux-amd64 && \
    chmod +x kind && mv kind /usr/local/bin/kind

# install kubectl
ARG KUBECTL_VERSION=v1.22.0
RUN curl -Lo kubectl https://storage.googleapis.com/kubernetes-release/release/v${KUBECTL_VERSION}/bin/linux/amd64/kubectl && \
    chmod +x kubectl && mv kubectl /usr/local/bin/kubectl

# install helm
RUN curl https://raw.githubusercontent.com/helm/helm/master/scripts/get-helm-3 | bash

# install skaffold
ARG SKAFFOLD_VERSION=1.32.0
RUN curl -Lo skaffold https://storage.googleapis.com/skaffold/releases/v${SKAFFOLD_VERSION}/skaffold-linux-amd64 && \
    chmod +x skaffold && mv skaffold /usr/local/bin/skaffold

# install k9s
RUN curl -sS https://webinstall.dev/k9s | /bin/bash && ln -s /root/.local/bin/k9s /usr/local/bin/k9s

# install dapr
ARG DAPR_VERSION=1.4.0
RUN curl -sS https://raw.githubusercontent.com/dapr/cli/master/install/install.sh | /bin/bash -s ${DAPR_VERSION}

# set kube config
ENV KUBECONFIG /kube/config
RUN mkdir -p /kube && chmod a+w /kube
