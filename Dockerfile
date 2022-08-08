#  
#  --------   ----------   -----
#  | rust |   | carbon |   | C |
#  --------   ----------   -----
#      |           |         |
#      |           |         |
FROM rust as rust
WORKDIR /usr/src/myapp
COPY ./src/lib/ .
RUN cargo build --release --all-targets --manifest-path /usr/src/myapp/Cargo.toml
#		   |	     |
# install carbon   |	     |
#	     ------|	     |
#	     |     |	     |
FROM linuxbrew/brew as brew
RUN HOMEBREW_MAKE_JOBS=8 HOMEBREW_VERBOSE=1 brew update
RUN HOMEBREW_MAKE_JOBS=8 HOMEBREW_VERBOSE=1 brew install python@3.9
RUN HOMEBREW_MAKE_JOBS=8 HOMEBREW_VERBOSE=1 brew install bazelisk
RUN HOMEBREW_MAKE_JOBS=8 HOMEBREW_VERBOSE=1 brew install llvm
#		   |	     |
#		   |	     |
#		   |	     |
#		   |	     |
  FROM brew as carbon
RUN git clone https://github.com/carbon-language/carbon-lang carbon
WORKDIR /home/linuxbrew/carbon
COPY --from=rust /usr/src/myapp/target/release/librust_file_listener.so /home/linuxbrew/carbon/explorer/
SHELL ["/bin/bash", "-c"] 
RUN mv -v /home/linuxbrew/carbon/explorer/BUILD /home/linuxbrew/carbon/explorer/BUILD-old
RUN touch ./explorer/BUILD
RUN sed -n '1,17p' ./explorer/BUILD-old >> ./explorer/BUILD
RUN echo '    srcs = ["main.cpp", "librust_file_listener.so"],' >> ./explorer/BUILD
RUN sed -n '19,$p' ./explorer/BUILD-old >> ./explorer/BUILD
RUN rm ./explorer/BUILD-old
RUN cp ./explorer/librust_file_listener.so .
RUN bazel build --jobs 8 --verbose_failures //explorer
COPY ./src/main.carbon .
COPY ./src/file-listener.h .
#RUN bazel run --jobs 8 //explorer -- ./main.carbon
#			     |
#			     |
#			     |
#			     |
#                FROM gcc as gcc
#WORKDIR /usr/src/myapp
#COPY ./src/main.c .
#COPY ./src/file-listener.h .
#RUN gcc main.c




#FROM alpine
#COPY --from=rust /usr/src/myapp/target/release/librust_file_listener.so .
