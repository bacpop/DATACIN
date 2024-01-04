import { RootState } from "@/store/state";
// import { WorkerMapper } from '@/workers/Mapper.worker.js';

export const emptyState = (): RootState => ({
    errors: [],
    allResults: {
        mapResults: {},
    },
    workerState: {
        worker: null,
        workerResult: null,
    },
});
