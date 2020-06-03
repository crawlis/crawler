#!/bin/bash

if [ $TARGET != "x86_64-unknown-linux-musl" ]
then
  rustup self update
  rustup target add ${TARGET}
fi