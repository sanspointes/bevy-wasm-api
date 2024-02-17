import { type MyApi, WasmVec3 } from 'bevy-app';

function buildEntityInspector(root: HTMLElement, api: MyApi, entityId: number) {
    const title = document.createElement('h4');
    title.innerText = `${entityId}: ...`;
    root.append(title);

    const info = document.createElement('h6');
    info.innerText = 'Using bevy world as the source of truth.'
    root.append(info);

    const updateEntityName = () => {
        api.get_entity_name(entityId).then((name) => {
            title.innerText = `${entityId}: ${name ?? 'Unnamed'}`
            nameInput.value = name ?? 'Unnamed';
        })
    }
    updateEntityName();

    const nameInput = document.createElement('input');
    nameInput.placeholder = 'Name';
    nameInput.value = '...';
    nameInput.addEventListener('input', async () => {
        await api.set_entity_name(entityId, nameInput.value);
        updateEntityName();
    })
    root.append(nameInput);

    // const positionContainer = document.createElement('div');
    // positionContainer.style.display = 'flex';
    // root.append(positionContainer);
    //
    // const [initialX, initialY, initialZ] = api.get_entity_position(entityId);
    // const inputX = document.createElement('input');
    // inputX.type = 'number';
    // inputX.step = '50';
    // inputX.value = `${initialX}`;
    // inputX.placeholder = 'X';
    // positionContainer.append(inputX);
    // const inputY = document.createElement('input');
    // inputY.type = 'number';
    // inputY.step = '50';
    // inputY.value = `${initialY}`;
    // inputY.placeholder = 'Y';
    // positionContainer.append(inputY);
    // const inputZ = document.createElement('input');
    // inputZ.type = 'number';
    // inputZ.step = '50';
    // inputZ.value = `${initialZ}`;
    // inputZ.placeholder = 'Z';
    // positionContainer.append(inputZ);
    //
    // function parseFloatFallback(floatString: string, fallback = 0) {
    //     try {
    //         return Number.parseFloat(floatString)
    //     } catch (_) {
    //         return fallback;
    //     }
    // }
    //
    // const handlePositionChanged = async () => {
    //     const x = parseFloatFallback(inputX.value);
    //     const y = parseFloatFallback(inputY.value);
    //     const z = parseFloatFallback(inputZ.value);
    //     console.log(`Trying to set entity ${entityId} to position ${x},${y},${z}`);
    //     try {
    //         await api.set_entity_position(entityId, x, y, z)
    //     } catch (reason) {
    //         console.error('Error while setting entity position', reason);
    //     }
    // }
    // inputX.addEventListener('change', handlePositionChanged);
    // inputY.addEventListener('change', handlePositionChanged);
    // inputZ.addEventListener('change', handlePositionChanged);
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
