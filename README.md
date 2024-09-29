# Walk The Dog

- Built following Game Development with Rust and WebAssembly book
- Repo:[https://github.com/jorge-gonzalez-lexia/walk-the-dog](https://github.com/jorge-gonzalez-lexia/walk-the-dog)
- Deployed to [https://jorge-walk-the-dog.netlify.app/](https://jorge-walk-the-dog.netlify.app/)
- Served from Netlify [https://app.netlify.com/sites/jorge-walk-the-dog/overview](https://app.netlify.com/sites/jorge-walk-the-dog/overview)

## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## How to run unit tests

```sh
# Runs tests in Firefox
npm test -- --firefox

# Runs tests in Chrome
npm test -- --chrome

# Runs tests in Safari
npm test -- --safari
```

## What does each file do?

- `Cargo.toml` contains the standard Rust metadata. You put your Rust dependencies in here. You must change this file with your details (name, description, version, authors, categories)

- `package.json` contains the standard npm metadata. You put your JavaScript dependencies in here. You must change this file with your details (author, name, version)

- `webpack.config.js` contains the Webpack configuration. You shouldn't need to change this, unless you have very special needs.

- The `js` folder contains your JavaScript code (`index.js` is used to hook everything into Webpack, you don't need to change it).

- The `src` folder contains your Rust code.

- The `static` folder contains any files that you want copied as-is into the final build. It contains an `index.html` file which loads the `index.js` file.

- The `tests` folder contains your Rust unit tests.
