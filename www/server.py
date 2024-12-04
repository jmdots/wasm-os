from http.server import HTTPServer, SimpleHTTPRequestHandler
import mimetypes

class WASMRequestHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        # Add WASM MIME type
        self.extensions_map.update({
            '.wasm': 'application/wasm',
            '.js': 'application/javascript',
            '.mjs': 'application/javascript'
        })

    def end_headers(self):
        # Add CORS headers
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        # Add Content-Type for modules
        if self.path.endswith('.js') or self.path.endswith('.mjs'):
            self.send_header('Content-Type', 'application/javascript')
        SimpleHTTPRequestHandler.end_headers(self)

if __name__ == '__main__':
    # Ensure mimetypes are registered
    mimetypes.add_type('application/javascript', '.js')
    mimetypes.add_type('application/javascript', '.mjs')
    mimetypes.add_type('application/wasm', '.wasm')
    
    server = HTTPServer(('', 8080), WASMRequestHandler)
    print('Starting server at http://localhost:8080')
    server.serve_forever()