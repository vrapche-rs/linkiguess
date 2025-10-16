FROM alpine:latest

RUN apk add curl
RUN apk add bash
RUN curl -fsSL https://bun.sh/install | bash
RUN apk add gcompat
RUN apk add libstdc++

RUN echo 'export BUN_INSTALL="$HOME/.bun"' >> ~/.bashrc
RUN echo 'export PATH="$BUN_INSTALL/bin:$PATH"' >> ~/.bashrc

RUN mkdir /root/project
WORKDIR /root/project

CMD /bin/bash -c "source ~/.bashrc && bun run dev"
