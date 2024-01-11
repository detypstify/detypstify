install_deps:
        npm install typescript

clean:
        echo "cleaning"
        rm -rf build_artifacts
        mkdir build_artifacts
build:
        echo "building"
        pushd recognize && wasm-pack build --out-dir ../build_artifacts/wasm --target web
        pushd webapp && ../node_modules/typescript/bin/tsc script.ts -m es2022 --lib es2022,dom
        mv webapp/script.js build_artifacts/script.js


