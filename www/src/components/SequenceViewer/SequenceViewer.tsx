import { defineComponent } from "vue";
import { useState } from "vuex-composition-helpers";
import SingleRow from './SingleRow.vue'
import { Rows, Row } from './SequenceViewerConstruction'
import { ref, h, computed } from 'vue'
import { useVirtualizer } from '@tanstack/vue-virtual'

export default defineComponent({
    name: "SequenceViewer",

    props: {
        zoom_level: {
            type: Number,
        },
        no_skip: {
            type: Boolean,
            default: false,
        },
    },
    
    setup(props: { zoom_level: Number | undefined, no_skip: boolean}){
        const { allResults } = useState(["allResults"]);
        const whole_sequences: string[] = allResults.value.ref;

        const whole_mapped_sequences_chrom: string[][] = [];
        for (let _ = 0; _ < whole_sequences.length; _++){
            whole_mapped_sequences_chrom.push([]);
        }
        for (const key in allResults.value.mapResults){
            for (let i = 0; i < whole_sequences.length; i++){
                whole_mapped_sequences_chrom[i].push(allResults.value.mapResults[key].mapped_sequences[i]);
            }
        }
        const parentRef = ref(null)
        const zoom_level = ref(props.zoom_level);

        const font_family = "Arial";
        const font_size: Number = zoom_level.value ? zoom_level.value : 10;

        const rows: Row[] = Rows(font_size, font_family, whole_mapped_sequences_chrom, whole_sequences, props.no_skip, Object.keys(allResults.value.mapResults));
        const rowVirtualizer = useVirtualizer({
            count: rows.length,
            getScrollElement: () => parentRef.value,
            estimateSize: () => (3 + rows[0].mapped_sequence.length)  * Number(font_size),
            overscan: 5,
        });

        const virtualRows = computed(() => rowVirtualizer.value.getVirtualItems());
        const totalSize = computed(() => rowVirtualizer.value.getTotalSize());
    
        return () => {
            return (
                h('div', {ref: parentRef, style: {height: "400px", overflow: 'auto'}},
                    [h('div', {
                        style:{ height: `${totalSize.value}px`, width: '100%', position: 'relative' }},
                        [virtualRows.value.map((virtualRow:{ index: number, size: number, start: number }) => {
                            return (
                                h('div', {
                                    key: virtualRow.index,
                                    style: {
                                        position: 'absolute',
                                        top: 0,
                                        left: 0,
                                        width: '100%',
                                        height: `${virtualRow.size}px`,
                                        transform: `translateY(${virtualRow.start}px)`,
                                    }
                                },
                                [h(SingleRow, { virtualRow, row: rows[virtualRow.index], font_size: font_size, font_family: font_family, mapping_names: Object.keys(allResults.value.mapResults) })])
                            );
                        })
                    ])
                ])
            );
        };
    },    
});