<template>
    <div v-if="queryProcessed" class="variants" style="text-align:left; margin-left:10px"> 
        <input type="checkbox" id="checkbox" v-model="checked"/>
        <label for="checkbox">See visualisation</label>
        <div v-if="!checked">
            <li v-for="filename in Object.keys(allResults.mapResults)" :key="filename"> 
                {{allResults.mapResults[filename]["nb_variants"] !== null ? "File: " + filename + " → Number of variants detected: " +  allResults.mapResults[filename]["nb_variants"] + ", Coverage: " + Math.round(allResults.mapResults[filename]['coverage']*100) + "%" : 'Loading...' }}
            </li>
        </div>
    </div>
</template>

<script>
import { useState } from "vuex-composition-helpers";
import { ref } from "vue";

export default {
    name: "ResultsDisplay",
    setup() {
        const { allResults } = useState(["allResults"]);
        const checked = ref(null);

        return {
            allResults,
            checked
        };
    },
    computed: {
        queryProcessed() {
            return this.$store.getters.queryProcessed;
        }
    }
};
</script>