<script setup>
import {onMounted, onUnmounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const time = ref("0:00");
const temperature = ref(0);

onMounted(() => {
  // Update the time every minute
  updateTime();

  const intervalId = setInterval(updateTime, 60000);

  onUnmounted(() => {
    clearInterval(intervalId); // Clear the interval when the component is unmounted
  });
})


function updateTime() {
  const now = new Date();
  time.value = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

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
  <div class="w-full py-2 px-4 bg-black bg-opacity-50 fixed top-0 z-10">
    <div class="flex justify-end items-center text-white text-md">
      <span class="mr-4">{{ Math.round(temperature * 10) / 10 }}°C</span>
      <!-- Time Display -->
      <span>{{ time }}</span>
    </div>
  </div>
</template>

<style scoped>

</style>