import {
    AllResults,
    WorkerState,
} from "@/types";

export interface RootState {
    refSet: string | null,
    allResults: AllResults
    workerState: WorkerState
}
