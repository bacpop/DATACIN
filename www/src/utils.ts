import { RootState } from "@/store/state";

export const emptyState = (): RootState => ({
    refSet: null,
    allResults: {
        mapResults: {},
        alignResults: {},
        ref: [],
    },
    workerState: {
        worker: null,
    },
});
