<template>
    <svg id="tree_container" style="height: 400px; width: 100%;"></svg>
</template>

<script>
import { useState } from "vuex-composition-helpers";
import * as d3 from 'd3';
import { NewickTools } from 'newick'

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
                if (this.allResults.alignResults[0]? this.allResults.alignResults[0].names.length > 1: false){
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

            // Create an SVG element
            const svg = d3.select("#tree_container");

            svg.append("text")
                .attr("x", "50%")
                .attr("y", 20)
                .text("Not enough alignments to visualise a tree")
                .attr("font-size", "15px")
                .attr("text-anchor", "middle");
        },
        createTree() {
            // Clear previous tree
            d3.select("#tree_container").selectAll("*").remove();

            let nwk = this.allResults.alignResults[0].newick;

            const treeData = NewickTools.parse(nwk);

            // Create a D3 tree layout
            const cluster = d3.tree()
                              .size([400, window.innerWidth - 400]);

            // Create an SVG element
            const svg = d3.select("#tree_container");
            
            let g = svg.append("g").attr("transform", "translate(40,0)");

            const root = d3.hierarchy(treeData, d => d.branchset);

            cluster(root);

            // Links
            g.selectAll(".link")
             .data(root.descendants().slice(1))
             .enter().append("path")
             .attr("class", "link")
             .attr("d", d => `
                M${d.y},${d.x}
                L${d.parent.y},${d.parent.x}`)
            .attr("stroke", "black")

            // Nodes
            const node = g.selectAll(".node")
                .data(root.descendants())
                .enter().append("g")
                .attr("class", "node")
                .attr("transform", d => `translate(${d.y},${d.x})`);

            node.append("circle")
                .attr("r", 2.5);

            node.append("text")
                .attr("dy", 3)
                .attr("x", d => d.children ? -8 : 8)
                .style("text-anchor", d => d.children ? "end" : "start")
                .text(d => d.data.name);

        },
    },

};
</script>