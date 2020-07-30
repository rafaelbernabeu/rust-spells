FROM ubuntu:20.04
MAINTAINER Rafael Bernabeu "rbbernabeu@gmail.com"

# --- GLOBAL CONFIG ---
RUN sed 's/main$/main universe/' -i /etc/apt/sources.list   
RUN apt update && apt upgrade -y

# Common
RUN apt install -y software-properties-common apt-transport-https apt-utils net-tools wget curl man-db

# Git
RUN apt install git -y

# Cargo
RUN apt install cargo -y

RUN mkdir -p /home/developer
RUN mkdir /home/developer/tools
RUN echo "developer:x:1000:1000:Developer,,,:/home/developer:/bin/bash" >> /etc/passwd
RUN echo "developer:x:1000:" >> /etc/group
RUN chown developer:developer -R /home/developer

USER developer

RUN cargo install bootimage
RUN echo 'export PATH=$PATH:/home/developer/.cargo/bin' >> ~/.bashrc

ENV HOME /home/developer
WORKDIR /home/developer
CMD /bin/bash
