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
        let results = this.SkaData.map(file, revReadFile);
        this.worker.postMessage({ nb_variants: results[0], coverage: results[0] / results[1], name: file.name.replace(/(.fasta|.fasta.gz|.fa|.fa.gz|.fq|.fq.gz|.fastq|.fastq.gz|_1.fq.gz|_1.fastq.gz)$/, '')});
    }

}