<template>
    <div v-if="alignmentProcessed">
        <div v-for="alignment in allResults.alignResults[0].pairwiseAlignment" :key="alignment">
            <p>{{ alignment }}</p>
        </div> 
    </div>
</template>

<script>
import { useState } from "vuex-composition-helpers";

export default {
    name: "ResultsDisplayAlignment",
    setup() {
        const { allResults } = useState(["allResults"]);

        return {
            allResults,
        }
    },

    computed: {
        alignmentProcessed() {
            return this.$store.getters.alignmentProcessed;
        },
    },
};
</script>