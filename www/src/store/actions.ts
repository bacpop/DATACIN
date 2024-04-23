import { ActionContext } from "vuex";
import { RootState } from "@/store/state";

export default {
    async processRef(context: ActionContext<RootState, RootState>, acceptFiles: Array<File>) {
        const { commit, state } = context;
        console.log("Ref file uploaded")
        acceptFiles.forEach((file: File) => {
            if (state.workerState.worker) {
                state.workerState.worker.postMessage({ref: true, file});
                state.workerState.worker.onmessage = () => {
                    console.log(file.name + " has been indexed");
                    commit("addRef", file.name);
                };
            }
        });
    },
    async processQuery(context: ActionContext<RootState, RootState>, acceptFiles: Array<File>) {
        const { commit, state } = context;
        console.log("Query files uploaded")

        const findReadPair = (fileName: string, files: Array<File>): { pairFile: File | undefined, sampleName: string } => {
            const baseName = fileName.replace(/(_1.fastq.gz|_1.fq.gz)$/, '');
            const pairNameFastq = baseName + '_2.fastq.gz';
            const pairNameFq = baseName + '_2.fq.gz';
            const pairFile = files.find(file => file.name === pairNameFastq || file.name === pairNameFq);
            return { pairFile, sampleName: baseName };
        };

        acceptFiles.forEach((file: File) => {
            let sendJob: boolean = false;
            const messageData: any = { map: true, file, revReads: null, sampleName: null };
            if (/(_1|_2)(.fastq.gz|.fq.gz)$/.test(file.name)) {
                const { pairFile, sampleName } = findReadPair(file.name, acceptFiles);
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
                messageData.sampleName = file.name.replace(/(.fasta|.fasta.gz|.fa|.fa.gz)$/, '');
                sendJob = true;
            }

            if (sendJob) {
                commit("addQueryFile", messageData.sampleName);
                if (state.workerState.worker) {
                    state.workerState.worker.postMessage(messageData);
                    state.workerState.worker.onmessage = (message) => {
                        console.log("Mapping result :" + message.data.mapping);
                        commit("setMapped", [message.data.name, message.data.mapping]);
                    };
                }
            }
        });
    },
};
