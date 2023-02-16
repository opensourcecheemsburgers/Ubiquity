const invoke = window.__TAURI__.invoke

export async function getSongs(path) {
    let result = await invoke("get_songs", {
        path: path
    })
    console.debug("Received result of type: " + typeof result);
    console.debug("Result: " + result);
    console.debug("Returning the result to yew")
    return result;
}