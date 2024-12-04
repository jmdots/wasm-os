import init, { create_shell } from './pkg/wasm_os.js';

async function run() {
    try {
        await init();
        const shell = await create_shell('terminal');
        shell.render();

        document.addEventListener('keydown', (event) => {
            // Prevent default behavior for some keys
            if (['Backspace', 'ArrowUp', 'ArrowDown', 'Tab'].includes(event.key)) {
                event.preventDefault();
            }

            let key = event.key;
            if (key === 'Enter') {
                key = 'Enter';
            } else if (key === 'Backspace') {
                key = 'Backspace';
            } else if (key.length === 1) {
                // Only allow printable characters
                if (key.match(/[\x20-\x7E]/)) {
                    key = event.key;
                } else {
                    return;
                }
            } else {
                return;
            }

            shell.handle_keypress(key);
        });
    } catch (e) {
        console.error('Failed to initialize:', e);
    }
}

run().catch(console.error);