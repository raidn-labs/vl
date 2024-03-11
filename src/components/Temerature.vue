<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const temperature = ref(0);

onMounted(() => {
  getTemperature();

  const intervalId = setInterval(getTemperature, 60000); // 60000 milliseconds = 1 minute

  onUnmounted(() => {
    clearInterval(intervalId); // Clear the interval when the component is unmounted
  });
})

async function getTemperature() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  temperature.value = await invoke("get_temperature");
}
</script>

<template>
  <div class="row" style="margin-top: 20px">
    <h1>Car temp: {{ Math.round(temperature * 10) / 10 }}</h1>
    <button class="btn" @click="getTemperature" style="width:100px; margin-left: 20px">Refresh</button>
  </div>
</template>
