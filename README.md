# server
```
cd server
npm install
node main.js
```
Server will be running at 127.0.0.1:3000

# client

## build the wasm lib
```
wasm-pack build
```

## build the client (making use of wasm lib)
```
npm install
export NODE_OPTIONS=--openssl-legacy-provider
npm start
```

Head to 127.0.0.1:8080 and launch the Developer console to check the result.
