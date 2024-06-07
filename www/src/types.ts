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

export interface Alignment {
    aligned: boolean
    names?: string[]
    newick?: string
}
export interface AllResults {
    alignResults: Dict<Alignment>
    mapResults: Dict<IsolateMapping>
    ref: string[]
}

