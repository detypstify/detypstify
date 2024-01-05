build:
        echo "building"
        rm -rf build_artifacts
        npm install typescript
        mkdir build_artifacts
        pushd recognize && wasm-pack build --out-dir ../build_artifacts/wasm --target web
        pushd webapp && ../node_modules/typescript/bin/tsc script.ts -m es2022 --lib es2022,dom
        mv webapp/script.js build_artifacts/script.js


