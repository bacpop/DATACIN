export type Dict<T> = Record<string, T>

export interface WorkerState {
    worker: Worker | null;
}

export interface Isolate {
    mapped: boolean
}

export interface AllResults {
    mapResults: Dict<Isolate>
}

