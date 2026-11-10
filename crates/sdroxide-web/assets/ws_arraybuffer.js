// Keeps incoming WebSocket binary frames on the ArrayBuffer path.
//
// ewebsock 0.8's web sender calls `socket.set_binary_type(Blob)` on every
// binary *send*. `binaryType` governs how *received* messages are delivered, so
// that one line silently switches the whole downlink to Blob the moment the
// client sends its first message — which it always does, at Hello.
//
// On the Blob path ewebsock reads each message through a fresh FileReader and
// finishes with `onloadend_cb.forget()`, which leaks the closure permanently.
// The server pushes spectrum, audio and meters at well over a hundred messages
// a second, so that is a hundred-plus leaked closures and FileReaders every
// second, each still holding its decoded buffer. A desktop browser has the
// headroom to hide it; an iPad tab is killed in a minute or two, which looks
// like the UI freezing and the server logging "remote session ended".
//
// Fixed upstream in no released version (0.8.0 is the latest), and ewebsock
// exposes no handle on the underlying socket, so neutralise the setter here
// instead. Only the harmful value is redirected; anything else passes through.
(function () {
    const desc = Object.getOwnPropertyDescriptor(WebSocket.prototype, "binaryType");
    if (!desc || !desc.set || !desc.get) return;
    Object.defineProperty(WebSocket.prototype, "binaryType", {
        configurable: true,
        enumerable: desc.enumerable,
        get: desc.get,
        set(value) {
            desc.set.call(this, value === "blob" ? "arraybuffer" : value);
        },
    });
})();
