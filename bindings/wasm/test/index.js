const http = require('http');
const fs = require('fs');
const path = require('path');
const url = require('url');

function mime(name) {
    const ext = path.extname(name);
    switch(ext) {
        case ".html":
            return "text/html";
        case ".js":
            return "text/javascript";
        case ".wasm":
            return "application/wasm";
        default:
            return "text/plain";
    }
}

function serveStatic(url, dir, res) {
    const pathname = path.join(dir, url.pathname);
    if(fs.existsSync(pathname) && fs.lstatSync(pathname).isFile()) {
        fs.createReadStream(pathname).pipe(res);
        res.setHeader('Content-Type', mime(pathname));
        return true;
    } else {    
        return false;
    }
}

const server = http.createServer((req, res) => {
    const parsedUrl = url.parse(req.url);
    serveStatic(parsedUrl, "./pkg", res);
    serveStatic(parsedUrl, "./test", res);
});

server.listen(8080, () => {
    console.log('Server listening on http://localhost:8080');
});