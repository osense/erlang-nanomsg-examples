### Introduction

This repository contains examples of using the IPC transport of nanomsg, where
Erlang is the client that sends requests and some other language is the worker
which replies to said requests.  As such, you should always first launch the
non-Erlang example first, so that it can bind to a IPC address and be ready
to accept connections. Only then launch the Erlang client.

The subdirectories here should all contain READMEs on how to compile & launch
them.
