export type Dict<T> = Record<string, T>

export interface WorkerState {
    worker: Worker | null;
}

export interface IsolateMapping {
    mapped: boolean
    nb_variants?: number | null
    coverage?: number | null
    mapped_sequences?: string[]
}

export interface IsolateAlignment {
    aligned: boolean
    alignment?: string
}
export interface AllResults {
    alignResults: Dict<IsolateAlignment>
    mapResults: Dict<IsolateMapping>
    ref: string[]
}

