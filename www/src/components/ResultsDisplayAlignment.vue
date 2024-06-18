<template>
    <svg id="tree_container"></svg>
</template>

<script>
import { useState } from "vuex-composition-helpers";
import * as phylotree from "phylotree";
import * as d3 from "d3";

export default {
    name: "ResultsDisplayAlignment",

    setup() {
        const { allResults } = useState(["allResults"]);

        return {
            allResults,
        }
    },

    props: {
        msg: String
    },

    watch: {
        'allResults.alignResults': {
            handler: function() {
                if (this.allResults.alignResults[0]? this.allResults.alignResults[0].names.length > 2: false){
                    this.createTree();
                }
                else {
                    this.notEnough();
                }
            },
            deep: true
        }
    },

    methods: { 
        notEnough() {
            // Clear previous tree
            d3.select("#tree_container").selectAll("*").remove();

            let container = d3.select("#tree_container");

            container.append("text")
                .attr("x", "50%")
                .attr("y", 20)
                .text("Not enough alignments to visualise a tree")
                .attr("font-size", "15px")
                .attr("text-anchor", "middle");
        },
        createTree() {
            // Clear previous tree
            d3.select("#tree_container").selectAll("*").remove();

            d3.select("#tree_container")
                .attr("width", window.innerWidth * 0.9)
                .attr("height", 400);

            let nwk = this.allResults.alignResults[0].newick;
            const tree = new phylotree.phylotree(nwk);
            console.log(tree);
            var rendered_tree = tree.render("#tree_container", {
                height: 400,
                width: window.innerWidth * 0.9,
                "left-right-spacing": "fit-to-size",
                "top-bottom-spacing": "fit-to-size"
            });
            console.log(rendered_tree);
        },
    },
};
</script>