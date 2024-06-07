import { ActionContext } from "vuex";
import { RootState } from "@/store/state";

export default {
    async processRef(context: ActionContext<RootState, RootState>, payload: { acceptFiles: Array<File>, k: number }) {
        const { commit, state } = context;
        console.log("Ref file uploaded, k = " + payload.k)
        payload.acceptFiles.forEach((file: File) => {
            if (state.workerState.worker) {
                state.workerState.worker.postMessage({ref: true, file, k : payload.k});
                state.workerState.worker.onmessage = (messageData) => {
                    console.log(messageData.data.ref.name + " has been indexed");
                    commit("addRef", {name: messageData.data.ref.name, sequences:messageData.data.sequences});
                };
            }
        });
    },
    async processQueryMap(context: ActionContext<RootState, RootState>, payload: {acceptFiles: Array<File>, proportion_reads: number}) {
        const { commit, state } = context;
        console.log("Query files uploaded mapping")
        const findReadPair = (fileName: string, files: Array<File>): { pairFile: File | undefined, sampleName: string } => {
            const baseName = fileName.replace(/(_1.fastq.gz|_1.fq.gz)$/, '');
            const pairNameFastq = baseName + '_2.fastq.gz';
            const pairNameFq = baseName + '_2.fq.gz';
            const pairFile = files.find(file => file.name === pairNameFastq || file.name === pairNameFq);
            return { pairFile, sampleName: baseName };
        };

        payload.acceptFiles.forEach((file: File) => {
            let sendJob: boolean = false;
            const messageData: any = { map: true, file, revReads: null, sampleName: null, proportion_reads: payload.proportion_reads };
            if (/(_1|_2)(.fastq.gz|.fq.gz)$/.test(file.name)) {
                const { pairFile, sampleName } = findReadPair(file.name, payload.acceptFiles);
                messageData.sampleName = sampleName;
                if (pairFile) {
                    messageData.revReads = pairFile;
                    sendJob = true;
                } else {
                    // Triggers on _2 input file too
                    if (/_1(.fastq.gz|.fq.gz)$/.test(file.name)) {
                        console.log(file.name + ": only one fastq found")
                    }
                }
            } else {
                messageData.sampleName = file.name.replace(/(.fasta|.fasta.gz|.fa|.fa.gz|.fq|.fastq)$/, '');
                sendJob = true;
            }

            if (sendJob) {
                commit("addQueryFileMap", messageData.sampleName);
                if (state.workerState.worker) {
                    state.workerState.worker.postMessage(messageData);
                    state.workerState.worker.onmessage = (message) => {
                        console.log("Mapping variants: " + message.data.nb_variants);
                        console.log("Mapping coverage: " + message.data.coverage);
                        commit("setMapped", message.data);
                    };
                }
            }
        });
    },

    async processQueryAlign(context: ActionContext<RootState, RootState>, payload: { acceptFiles: Array<File>, k: number, proportion_reads: number }) {
        const { commit, state } = context;
        console.log("Query files uploaded alignment")

        const messageData = { align: true, files: payload.acceptFiles, k: payload.k, proportion_reads: payload.proportion_reads};
        
        if (state.workerState.worker) {
            state.workerState.worker.postMessage(messageData);
            state.workerState.worker.onmessage = (message) => {
                commit("setAligned", message.data);
            };
        }
    },

    async resetAllResults(context: ActionContext<RootState, RootState>) {
        const { commit } = context;
        commit("resetAllResults");
    }
};
