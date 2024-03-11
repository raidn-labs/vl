<script setup>
import Temperature from "./components/Temerature.vue";
import {invoke} from "@tauri-apps/api/tauri";

async function setFanSpeed(speed) {
  await invoke("set_fan_speed", { speed });
}
</script>

<template>
  <div class="container">
    <div class="row">
      <button class="btn mr-10 h-32 w-32" @click="setFanSpeed(0)">Off</button>
      <button class="btn mr-10 h-32 w-32" @click="setFanSpeed(1)">Low</button>
      <button class="btn mr-10 h-32 w-32" @click="setFanSpeed(2)">Med</button>
      <button class="btn h-32 w-32" @click="setFanSpeed(3)">High</button>
    </div>
    <Temperature />
  </div>
</template>

<style scoped>

</style>
