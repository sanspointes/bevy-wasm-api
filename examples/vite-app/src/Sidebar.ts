import { type MyApi } from 'bevy-app';

export default function buildSidebar(element: HTMLElement, api: MyApi) {
    const responseText = document.createElement('p');
    const refreshButton = document.createElement('button');
    refreshButton.innerText = 'Refresh';
    refreshButton.addEventListener('click', async () => {
        console.log('Refreshing entities...');
        const numEntities = await api.count_entites();
        console.log(`Found ${numEntities} entitie(s)`);
        responseText.innerText = `${numEntities} entitie(s)`;
    })
    element.append(refreshButton);
    element.append(responseText);

    const list = document.createElement('div');
    element.append(list);
    // TODO: Entity inspector
}
