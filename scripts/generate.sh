#!/bin/sh
# http://rwlgt-iiaaa-aaaaa-aaaaa-cai.localhost:8080/dirs
canisters=(
    "file_manager"
)

echo -e "${GREEN}> $ENV: Generating required files..${NC}"
cargo test --test generate -q

for t in ${canisters[@]}; do
    echo -e "${GREEN} $ENV > Building $t..${NC}"
    dfx build --network ic $t
    dfx generate

    mkdir -p frontend/$t
    cp -a src/declarations/$t frontend
    rm -rf src/declarations
done

echo -e "${GREEN} $ENV > Stopping local replica..${NC}"