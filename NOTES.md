The app uses `webpack` to transpile the TypeScript and pack the web files for distribution.

*The following commands are all run from the `www` folder.*

# Development
1. Build the Rust code. 
```shell
> pnpm build
```
2. Run the front end without bundling it
```shell
> pnpm start
```
The app is served at the following URLs:
- Loopback: 
  - [http://localhost:8080/](http://localhost:8080/) (IPv4)
  - [http://[::1]:8080/](http://[::1]:8080/) (IPv6)
- On Your Network: 
  - [http://192.168.1.12:8080/](http://192.168.1.12:8080/) (IPv4)
  - [http://[fd22:822e:747c:7b42:9f09:3595:1729:40a1]:8080/](http://[fd22:822e:747c:7b42:9f09:3595:1729:40a1]:8080/) (IPv6)

# Distribution
1. Build the Rust code.
```shell
> pnpm build
```
2. Bundle the front end for distribution. Configured in `webpack.config.js` to put files in `dist` folder.
```shell
> pnpm bundle
```
To install locally, copy the dist folder and its contents to the target folder.
```shell
$ sudo rcp -r ./dist /var/www/html/wasm-life
```
To update locally, copy the files in the dist folder to the target folder.
```shell
$ sudo rcp -r ./dist/* /var/www/html/wasm-life
```
To install on tilde.team the first time, copy the dist folder and its contents to the target folder.
```shell
$ rcp -r ./dist tilde.team:~/public_html/wasm-life
```
To update the files on tilde.team, copy the files in the dist folder to the target folder.
```shell
$ rcp -r ./dist/* tilde.team:~/public_html/wasm-life
```