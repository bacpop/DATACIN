export class Mapper {
    worker;
    wasm;
    SkaRef;

    constructor(worker) {
        this.worker = worker;
        this.SkaRef = null;
        this.wasm = null;
        this.wasmPromise = new Promise(resolve => {
            import("@/pkg")
                .then((w) => {
                    this.wasm = w;
                    resolve(w);
                });
        });
    }

    waitForWasm() {
        return this.wasm ? Promise.resolve(this.wasm) : this.wasmPromise;
    }

    async set_ref(filename) {
        await this.waitForWasm();

        if (this.SkaRef === null) {
            this.SkaRef = this.wasm.SkaRef.new(filename);
        }
        this.worker.postMessage({ ref: filename });
    }

    map(filename, revReadFile) {
        if (this.SkaRef === null) {
            throw new Error("SkaRef::map - reference does not exist yet.");
        }
        this.worker.postMessage({ mapping: this.SkaRef.map(filename, revReadFile) });
    }

}