# Henver-R

This is a simple HTTP server build with Rust.<br />
It is used to host static websites created with React, Vue.js, etc.

## Usage

Use the following command to start the server.

```sh
cargo run
```

## Config

- The config file is `/config/config.toml`, you can adjust the settings in this file.
- All webpage and files should be deployed in folder `/public` (e.g. index.html, favicon.ico, etc.).
- You can place the error pages in folder `/public/error_pages`, or you can change the settings in `config.toml`.

## Todo

### Speed

- [ ] Prefork
- [ ] File Caching.
- [ ] Banned IP filter re-write.

### Error

- [ ] Server hangs up when receiving URIs that are too long.

## License

<a href="https://github.com/91d906h4/Henver-R/blob/main/LICENSE">MIT License</a>