const download = require('download');
const decompress = require('decompress');
const { version } = require('../package.json')

const root = 'https://github.com/yozuk/yozuk/releases/download';
const files = [
    `${root}/v${version}/yozuk-wasm-web.tar.gz`,
    `${root}/v${version}/yozuk-wasm-web-debug.tar.gz`,
];

(async () => {
    await Promise.all(files.map(async (file) => await decompress(await download(file), '.')));
})()