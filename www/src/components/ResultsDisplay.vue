<template>
    <div v-if="queryProcessed" class="variants" style="text-align:left; margin-left:10px"> 
        <input type="checkbox" id="visualisation" v-model="visualisation"/>
        <label for="visualisation">See visualisation</label>
        <div v-if="!visualisation">
            <li v-for="filename in Object.keys(allResults.mapResults)" :key="filename"> 
                {{allResults.mapResults[filename]["nb_variants"] !== null ? 
                        "File: " + filename + " → Number of variants detected: " +  allResults.mapResults[filename]["nb_variants"] + ", Coverage: " + Math.round(allResults.mapResults[filename]['coverage']*100) + "%" 
                        : 'Loading...' }}
            </li>
        </div>
        <div v-else>
            <div v-if="zoom>8">
                <input type="checkbox" id="skip" v-model="skip"/>
                <label for="skip">Skip unmapped sequences</label>
            </div>
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
            <div v-if="zoom>8">
                <SequenceViewer 
                    :zoom_level="zoom"
                    :no_skip="skip"
                    :key="use_keys([zoom, skip, reloadKey])"> <!-- Reactivity on zoom and skip changes and reloadr -->
                </SequenceViewer>
            </div>
            <div v-else>
                <MinimisedSequenceViewer 
                    :zoom_level="zoom"
                    :key="use_keys([zoom, reloadKey])"> <!-- Reactivity on zoom changes and reload -->
                </MinimisedSequenceViewer>
            </div>
        </div>  
    </div>
</template>

<script>
import { useState } from "vuex-composition-helpers";
import { ref } from "vue";
import SequenceViewer from "./SequenceViewer/SequenceViewer.tsx";
import VueSlider from 'vue-3-slider-component'
import MinimisedSequenceViewer from "./MinimisedSequenceViewer/MinimisedSequenceViewer.vue";

export default {
    name: "ResultsDisplay",
    components: {
        SequenceViewer,
        VueSlider,
        MinimisedSequenceViewer
    },
    setup() {
        const { allResults } = useState(["allResults"]);
        const visualisation = ref(false);
        const skip = ref(false);

        return {
            allResults,
            visualisation,
            skip
        }
    },

    watch: {
        'allResults.mapResults': {
            handler() {
                let last_key = Object.keys(this.allResults.mapResults)[Object.keys(this.allResults.mapResults).length-1]
                if (this.allResults.mapResults[last_key].mapped_sequences.length !== 0){
                    this.reloadKey++;
                }
            },
            deep: true,
        },
    },

    computed: {
        queryProcessed() {
            return this.$store.getters.queryProcessed;
        }
    },

    methods: {
        use_keys(list_of_keys) {
            return list_of_keys.join('-');
        }
    },
    
    data() {
        return {
            zoom: 10,
            reloadKey: 0
        }
    },
};
</script>