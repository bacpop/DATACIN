<template>
    <div v-if="queryProcessed" class="variants" style="text-align:left; margin-left:10px"> 
        <input type="checkbox" id="checkbox" v-model="checked"/>
        <label for="checkbox">See visualisation</label>
        <div v-if="!checked">
            <li v-for="filename in Object.keys(allResults.mapResults)" :key="filename"> 
                {{allResults.mapResults[filename]["nb_variants"] !== null ? "File: " + filename + " → Number of variants detected: " +  allResults.mapResults[filename]["nb_variants"] + ", Coverage: " + Math.round(allResults.mapResults[filename]['coverage']*100) + "%" : 'Loading...' }}
            </li>
        </div>
        <div v-else>
            <VueSlider 
                v-model="zoom" 
                :lazy="true" 
                :min="1"
                :max="20"
                :interval="0.1"
                :tooltip="'none'"
                style="margin: 10px 0"
                >
            </VueSlider>
            <SequenceViewer 
                :zoom_level="zoom"
                :key="zoom">
            </SequenceViewer>
        </div>  
    </div>
</template>

<script>
import { useState } from "vuex-composition-helpers";
import { ref } from "vue";
import SequenceViewer from "./SequenceViewer/SequenceViewer.tsx";
import VueSlider from 'vue-3-slider-component'

export default {
    name: "ResultsDisplay",
    components: {
        SequenceViewer,
        VueSlider
    },
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
    },
    
    data() {
        return {
        zoom: 5
        }
    },
};
</script>