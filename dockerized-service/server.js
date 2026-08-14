require('dotenv').config();
const http = require('http');
const fs = require('fs');
const path = require('path');

const renderLoginForm = (res, error = '') => {
  const filePath = path.join(__dirname, 'login.html');
  fs.readFile(filePath, 'utf8', (err, html) => {
    if (err) {
      res.writeHead(500, { 'Content-Type': 'text/plain' });
      return res.end('Server Error');
    }

    const errorHtml = error ? `<p style="color:red;">${error}</p>` : '';
    const rendered = html.replace('{{ERROR}}', errorHtml);

    res.writeHead(200, { 'Content-Type': 'text/html' });
    res.end(rendered);
  });
};

const server = http.createServer((req, res) => {
  if (req.url === '/') {
    res.writeHead(200, { 'Content-Type': 'text/plain' });
    return res.end('Hello, world!');
  }

  if (req.url === '/secret') {
    if (req.method === 'GET') {
      return renderLoginForm(res);
    }

    if (req.method === 'POST') {
      let body = '';
      req.on('data', chunk => { body += chunk.toString(); });
      req.on('end', () => {
        const params = new URLSearchParams(body);
        const username = params.get('username');
        const password = params.get('password');

        if (username === process.env.USERNAME && password === process.env.PASSWORD) {
          res.writeHead(200, { 'Content-Type': 'text/plain' });
          return res.end(process.env.SECRET_MESSAGE);
        }

        renderLoginForm(res, 'Invalid username or password.');
      });
      return;
    }
  }

  res.writeHead(404, { 'Content-Type': 'text/plain' });
  res.end('Not Found');
});

server.listen(3000, () => {
  console.log('Server running on http://localhost:3000');
});
