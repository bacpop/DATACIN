<template>
    <div v-if="queryProcessed" class="variants" style="text-align:left; margin-left:10px"> 
        <li v-for="filename in Object.keys(allResults.mapResults)" :key="filename"> 
            {{allResults.mapResults[filename]["nb_variants"] !== null ? "File: " + filename + " → Number of variants detected: " +  allResults.mapResults[filename]["nb_variants"] + ", Coverage: " + Math.round(allResults.mapResults[filename]['coverage']*100) + "%" : 'Loading...' }}
        </li>
    </div>
</template>

<script>
import { useState } from "vuex-composition-helpers";

export default {
    name: "ResultsDisplay",
    setup() {
        const { allResults } = useState(["allResults"]);

        return {
            allResults
        };
    },
    computed: {
        queryProcessed() {
            return this.$store.getters.queryProcessed;
        }
    }
};
</script>