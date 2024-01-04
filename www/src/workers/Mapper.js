export class Mapper {
    worker;
    wasm;
    ref;

    constructor(worker) {
        this.worker = worker;
        this.ref = null;
        this.wasm = null;
        import("@/pkg")
            .then((w) => {
                this.wasm = w;
                this.worker.postMessage({ on_initialized: true });
            });
    }

    set_ref(filename) {
        if (this.wasm === null) {
            throw new Error("Wasm is not initialized in SkaRef::new()");
        }
        if (this.ref === null) {
            this.ref = this.wasm.SkaRef.new(filename);
        }
        this.worker.postMessage("Ref set to " + this.ref);
    }

    map(filename) {
        if (this.ref === null) {
            throw new Error("SkaRef::map - reference does not exist yet.");
        }
        this.worker.postMessage({ mapping: this.ref.map(filename) });
    }

}