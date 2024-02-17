import init, { MyApi, setup_bevy_app } from 'bevy-app';
import Toolbar from './Toolbar';
import Sidebar from './Sidebar';

async function main() {
    await init();
    try {
        setup_bevy_app('#canvas');
    } catch(error) {
        // Dont worry about it, the app runner errors.
    }

    const api = new MyApi();

    Toolbar(document.getElementById('toolbar')!, api);
    Sidebar(document.getElementById('sidebar')!, api);
}

main();
