
<template>
    <div :id="'parent'"></div>
</template>


<script>
import * as d3 from 'd3'; 
import { useState } from 'vuex-composition-helpers';

export default {
    props: ['zoom_level'],

    mounted() {
        this.createSequenceViewer();
    },

    methods: {
        createSequenceViewer() {
            const { allResults } = useState(["allResults"]);

            const whole_sequences = allResults.value.ref;

            const whole_mapped_sequences_chrom = [];
            for (let _ = 0; _ < whole_sequences.length; _++){
                whole_mapped_sequences_chrom.push([]);
            }
            for (const key in allResults.value.mapResults){
                for (let i = 0; i < whole_sequences.length; i++){
                    whole_mapped_sequences_chrom[i].push(allResults.value.mapResults[key].mapped_sequences[i]);
                }
            }

            const nb_mapping = whole_mapped_sequences_chrom[0].length;

            let length_sequence = 0;
            for (const seq of whole_sequences) {
                length_sequence += seq.length;
            }
            const nb_chrom = whole_sequences.length;

            const width = document.body.clientWidth - 20;
            const totalWidth = width * this.zoom_level;
            const height = 420;
            const marginTop = 20;
            const marginRight = 20;
            const marginBottom = 30;
            const marginLeft = 30;

            // Create the horizontal (x) scale over the total width.
            const x = d3.scaleLinear()
                .domain([0, length_sequence + (nb_chrom - 1) * 100])
                .range([marginLeft, totalWidth - marginRight]);

            // Create the vertical (x) scale.
            const y = d3.scaleLinear()
                .domain([0, nb_mapping])
                .range([height - marginBottom, marginTop]);

            // Create a div that holds two svg elements: one for the main chart and horizontal axis,
            // which moves as the user scrolls the content; the other for the vertical axis (which 
            // doesn’t scroll).
            const parent = d3.select('#parent');
            console.log("parent");

            // Create the svg with the vertical axis. 
            parent.append("svg")
                .attr("width", width)
                .attr("height", height)
                .style("position", "absolute")
                .style("pointer-events", "none")
                .style("z-index", 1)
                .style("left", 0)
                .append("g")
                .attr("transform", `translate(${marginLeft},0)`)
                .call(d3.axisLeft(y))
                .call(g => g.select(".domain").remove())

            // Create a scrolling div containing the area shape and the horizontal axis. 
            const body = parent.append("div")
                .style("overflow-x", "scroll")
                .style("-webkit-overflow-scrolling", "touch");

            const svg = body.append("svg")
                .attr("width", totalWidth)
                .attr("height", height)
                .style("display", "block");
            
            for (let map_i = 0; map_i < nb_mapping; map_i++) {
                let nucleotide_count = 0;
                for (let chr_i = 0; chr_i < nb_chrom; chr_i++) {

                    let current_nucleotide = nucleotide_count;
                    let current_is_equal = whole_mapped_sequences_chrom[chr_i][map_i][0] == "-"? null : whole_mapped_sequences_chrom[chr_i][map_i][0] == whole_sequences[chr_i][0];

                    for (let nuc_i = 0; nuc_i < whole_sequences[chr_i].length; nuc_i++) {
                        if (current_is_equal == null) {
                            current_is_equal = whole_mapped_sequences_chrom[chr_i][map_i][nuc_i] == "-"? null : whole_mapped_sequences_chrom[chr_i][map_i][nuc_i] == whole_sequences[chr_i][nuc_i];
                            nucleotide_count += 1;
                            current_nucleotide = nucleotide_count;
                            continue
                        }
                        let new_is_equal = whole_mapped_sequences_chrom[chr_i][map_i][nuc_i] == "-"? null : whole_mapped_sequences_chrom[chr_i][map_i][nuc_i] == whole_sequences[chr_i][nuc_i];
                        if ((new_is_equal && current_is_equal) || (!new_is_equal && !current_is_equal)) {
                            current_is_equal = new_is_equal; 
                        }
                        else {
                            svg.append("rect")
                                .attr("x", x(current_nucleotide + chr_i * 100))
                                .attr("y", y(map_i+1))
                                .attr("width", x(nucleotide_count + chr_i * 100) - x(current_nucleotide + chr_i * 100))
                                .attr("height", y(map_i) - y(map_i + 1))
                                .attr("fill", current_is_equal ? "black" : "red");
                            current_nucleotide = nucleotide_count;
                            current_is_equal = new_is_equal;
                        }
                        nucleotide_count += 1;
                    }
                    svg.append("rect")
                        .attr("x", x(current_nucleotide + chr_i * 100))
                        .attr("y", y(map_i+1))
                        .attr("width", x(nucleotide_count + chr_i * 100) - x(current_nucleotide + chr_i * 100))
                        .attr("height", y(map_i) - y(map_i + 1))
                        .attr("fill", current_is_equal ? "black" : "red");
                }
            }
        }
    }
};
</script>