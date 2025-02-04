FROM docker-public.binary.picodata.io/tarantool:2.10.0

RUN set -e; \
    yum -y install gcc git; \
    yum clean all;

ENV PATH=/root/.cargo/bin:${PATH}
RUN set -e; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |\
    sh -s -- -y --profile default --default-toolchain 1.61.0 -c rustfmt -c clippy;

COPY ci-log-section /usr/bin/ci-log-section
