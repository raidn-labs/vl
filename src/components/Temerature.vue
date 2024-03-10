<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const temperature = ref(null);

async function getTemperature() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  temperature.value = await invoke("get_temperature");
}
</script>

<template>
  <div class="row" style="margin-top: 20px">
    <button class="btn" @click="getTemperature" style="width:150px; margin-right: 20px">Get Temp</button>
    <h1>{{ Math.round(temperature * 10) / 10 }}</h1>
  </div>
</template>
