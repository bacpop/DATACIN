export type Dict<T> = Record<string, T>

export interface WorkerState {
    worker: Worker | null;
}

export interface Isolate {
    mapped: boolean
    nb_variants?: number | null
    coverage?: number | null
    mapped_sequences?: string[]
}

export interface AllResults {
    mapResults: Dict<Isolate>
    ref: string[]
}

