<template>
    <div v-if="queryProcessed" class="variants"> 
        <div v-if="filesUploaded" class="checkbox">
            <input type="checkbox" id="visualisation" v-model="visualisation"/>
            <label for="visualisation">See visualisation</label>
        </div>
        <div v-if="!visualisation" id="table">
            <li v-for="filename in Object.keys(allResults.mapResults)" :key="filename"> 
                {{allResults.mapResults[filename]["nb_variants"] !== null ? 
                        "File: " + filename + " → Number of variants detected: " +  allResults.mapResults[filename]["nb_variants"] + ", Coverage: " + Math.round(allResults.mapResults[filename]['coverage']*100) + "%" 
                        : 'Loading...' }}
            </li>
        </div>
        <div v-else>
            <div v-if="zoom>8" class="checkbox">
                <input type="checkbox" id="skip" v-model="skip"/>
                <label for="skip">Skip unmapped sequences</label>
            </div>
            <div v-if="zoom>8">
                <Popper>
                    <button class="legend">?</button>
                    <template #content>
                        <div style="text-align: left;">
                            <b style="color: red; display: inline-block; width: 60px;">A</b> Diffence between the reference and the mapped sequence<br>
                            <b style="display: inline-block; width: 60px;">-</b> Similarity between the reference and the mapped sequence<br>
                            <div style="display: inline-block; width: 60px; font-size: 10px;">Blank space</div> Unmapped nucleotide<br>
                            <b style="display: inline-block; width: 60px;">.......</b> Skipped unmapped fraction of the sequence<br>
                        </div>
                    </template>
                </Popper>
            </div>
            <div v-else>
                <Popper>
                    <button class="legend">?</button>
                    <template #content>
                        <div style="text-align: left;">
                            <div class="square" style="background-color: red;"></div> Part of the sequence different to the reference<br>
                            <div class="square" style="background-color: black;"></div> Part of the sequence similar to the reference<br>
                            <div class="square" style="background-color: white;"></div> Part of the sequence not mapped<br>
                        </div>
                    </template>
                </Popper>
            </div>
            <VueSlider 
                v-model="zoom" 
                :lazy="true" 
                :min="1"
                :max="20"
                :interval="0.1"
                :tooltip="'none'"
                style="margin: 5px 0"
                >
            </VueSlider>
            <div v-if="zoom>8">
                <SequenceViewer 
                    :zoom_level="zoom"
                    :no_skip="!skip"
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
import Popper from "vue3-popper";

export default {
    name: "ResultsDisplay",
    components: {
        SequenceViewer,
        VueSlider,
        MinimisedSequenceViewer,
        Popper
        },
    setup() {
        const { allResults } = useState(["allResults"]);
        const visualisation = ref(false);
        const skip = ref(true);

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
        },
        filesUploaded() {
            let last_key = Object.keys(this.allResults.mapResults)[Object.keys(this.allResults.mapResults).length-1]
            if (this.allResults.mapResults[last_key].mapped_sequences.length !== 0){
                return true;
            }
            return false;
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

<style>
  :root {
    --popper-theme-background-color: lightgray;
    --popper-theme-background-color-hover: lightgray;
    --popper-theme-text-color: black;
    --popper-theme-border-width: 3px;
    --popper-theme-border-style: solid;
    --popper-theme-border-radius: 6px;
    --popper-theme-padding: 5px;
  }

  .checkbox {
    width: 30%;
    float: left;
    text-align: left;
  }

  .legend {
    background-color: #333333;
    color: white;
    border: none;
    cursor: pointer;
    padding: 5px 10px;
    text-decoration: none;
    display: inline-block;
    font-size: 14px;
    transition-duration: 0.4s;
    border-radius: 100px;
  }

  #table {
    width: 100%;
    float: left;
    margin-top: 10px;
    text-align: left;
    margin-left: 20px;
  }

  .square {
    height: 15px;
    width: 5px;
    margin-right: 30px;
    float: left;
  }
</style>