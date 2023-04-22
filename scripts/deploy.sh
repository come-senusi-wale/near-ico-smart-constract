#!/bin/bash


near delete ico.akinyemisaheedwale2.testnet akinyemisaheedwale2.testnet

near create-account ico.akinyemisaheedwale2.testnet --masterAccount akinyemisaheedwale2.testnet --initialBalance 60


near deploy --wasmFile=./res/ico.wasm --accountId  ico.akinyemisaheedwale2.testnet