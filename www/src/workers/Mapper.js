export class Mapper {
    worker;
    wasm;
    SkaRef;

    constructor(worker) {
        this.worker = worker;
        this.SkaData = null;
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

    async set_ref(file) {
        await this.waitForWasm();

        if (this.SkaData === null) {
            this.SkaData = this.wasm.SkaData.new(file);
        }
        this.worker.postMessage({ ref: file });
    }

    map(file, revReadFile) {
        if (this.SkaData === null) {
            throw new Error("SkaRef::map - reference does not exist yet.");
        }
        this.worker.postMessage({ mapping: this.SkaData.map(file, revReadFile) });
    }

}