import { type MyApi } from 'bevy-app';

function buildEntityInspector(root: HTMLElement, api: MyApi, entityId: number) {
    const title = document.createElement('h2');
    title.innerText = `Entity ${entityId}`;
    root.append(title);
}

export default function buildSidebar(element: HTMLElement, api: MyApi) {
    const list = document.createElement('div');

    const responseText = document.createElement('p');
    const refreshButton = document.createElement('button');
    refreshButton.innerText = 'Refresh';
    refreshButton.addEventListener('click', async () => {
        while (list.firstChild) {
            list.firstChild.remove();
        }

        const entities = await api.get_entities();
        responseText.innerText = `${entities.length} entitie(s)`;

        for (const id of entities) {
            const inspectorEl = document.createElement('div');
            buildEntityInspector(inspectorEl, api, id);
            list.append(inspectorEl);
        }
    })
    element.append(refreshButton);
    element.append(responseText);

    element.append(list);
}
