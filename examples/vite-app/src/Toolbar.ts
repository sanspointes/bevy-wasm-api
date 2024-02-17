import { type MyApi } from 'bevy-app';

export default function buildToolbar(element: HTMLElement, api: MyApi) {
    // Title
    const title = document.createElement('h1');
    title.innerText = 'Bevy Wasm Api vite-app';
    element.append(title);

    // Spawn box button
    const inputX = document.createElement('input');
    inputX.type = 'number';
    inputX.step = '50';
    inputX.value = '0';
    inputX.placeholder = 'X';
    element.append(inputX);
    const inputY = document.createElement('input');
    inputY.type = 'number';
    inputY.step = '50';
    inputY.value = '0';
    inputY.placeholder = 'Y';
    element.append(inputY);
    const inputZ = document.createElement('input');
    inputZ.type = 'number';
    inputZ.step = '50';
    inputZ.value = '0';
    inputZ.placeholder = 'Z';
    element.append(inputZ);

    const responseText = document.createElement('span');
    const button = document.createElement('button');
    button.innerText = 'Spawn circle';
    button.addEventListener('click', async () => {
        const x = Number.parseFloat(inputX.value);
        const y = Number.parseFloat(inputY.value);
        const z = Number.parseFloat(inputZ.value);
        console.log(`Spawning a circle at ${x},${y},${z}.`);
        const response = await api.spawn_circle(x, y, z)
        responseText.innerText = `Spawned circle, entity ${response}`;
    });
    element.append(button);
    element.append(responseText);

    console.log(element);
}
