// This will be Instance B of our Wasm module
let read_shared;

self.onmessage = async (event) => {
    const { type, wasmBytes, memory } = event.data;

    if (type === 'INIT_WASM') {
        console.log("[Worker JS] Received INIT_WASM message.");
        if (!wasmBytes || !memory) {
            console.error("[Worker JS] Wasm bytes or memory not received.");
            self.postMessage("Error: Wasm bytes or memory not provided to worker.");
            return;
        }
        try {
            console.log("[Worker JS] Importing wasm-bindgen glue code...");
            const initModule = await import('./pkg/wasm_shared.js');
            
            console.log("[Worker JS] Initializing Wasm module (Instance B) in worker...");
            // Initialize the Wasm module with the received bytes and SHARED memory
            await initModule.default({wasmBytes, memory});
            console.log("[Worker JS] Wasm module (Instance B) initialized in worker.");

            // Assign the specific Rust functions we need from the initialized module
            read_shared = initModule.read_shared_from_worker;


            self.postMessage("Worker Wasm (Instance B) initialized successfully.");
        } catch (error) {
            console.error("[Worker JS] Error initializing Wasm in worker:", error);
            self.postMessage(`Error initializing Wasm in worker: ${error.message} \nStack: ${error.stack}`);
        }
    } else if (type === 'READ_STRUCT') {
        console.log("[Worker JS] Received READ_STRUCT message.");
        try {
            const result = read_shared();
            console.log("[Worker JS] Rust function executed. Result:", result);
            self.postMessage(result);
        } catch (error) {
            console.error("[Worker JS] Error calling Rust function from worker:", error);
            self.postMessage(`Error in worker Rust call: ${error}`);
        }
    }
};

console.log("[Worker JS] Script loaded.");
