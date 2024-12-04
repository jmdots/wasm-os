from http.server import HTTPServer, SimpleHTTPRequestHandler

class WASMRequestHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        # Add WASM MIME type
        self.extensions_map.update({
            '.wasm': 'application/wasm',
            '.js': 'text/javascript',
        })

    def end_headers(self):
        # Add CORS headers
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        SimpleHTTPRequestHandler.end_headers(self)

if __name__ == '__main__':
    server = HTTPServer(('', 8080), WASMRequestHandler)
    print('Starting server at http://localhost:8080')
    server.serve_forever()