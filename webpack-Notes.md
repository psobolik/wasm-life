# Webpack Notes

May 2025

_Note that Webpack copies the `*.wasm` file to `dist` because of includes, but it knows nothing about the WASM code 
behind it, which has to be be rebuilt whenever it changes._  

Each of the following loaders/plugins is configured in `webpack.config.js`.

- Webpack uses [ts-loader](https://github.com/TypeStrong/ts-loader) to transpile and load TypeScript 
files on the fly during development. When the site is built for production, Webpack transpiles
TypeScript files and copies them to the `dist` folder.

- Webpack uses [style-loader](https://webpack.js.org/loaders/style-loader/#root) to inject stylesheet into the DOM, but 
only in development. When the site is built it uses `mini-css-extract-plugin` instead.

- Webpack uses [mini-css-extract-plugin](https://webpack.js.org/plugins/mini-css-extract-plugin/), 
which "…extracts CSS into separate files. It creates a CSS file per JS file which contains CSS. 
It supports On-Demand-Loading of CSS and SourceMaps."

- Webpack uses [css-loader](https://webpack.js.org/loaders/css-loader/) to detect and resolve `@import` and `url()` in 
code.

- Webpack uses the [html-webpack-plugin](https://github.com/jantimon/html-webpack-plugin#options) to 
generate the `index.html` file when it is needed by the server or when the project is built. 
The generated file contains any script tags the page will need. Because it specifies `favicon.svg` in the configuration, that file is
copied to the output folder, `dist`, when the project is built. We need the html page to have lots of content,
so we are specifying a custom template: `templates/index.html`.