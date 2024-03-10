<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Temperature from "./components/Temerature.vue";
import {invoke} from "@tauri-apps/api/tauri";

async function setFanSpeed(speed) {
  await invoke("set_fan_speed", { speed });
}
</script>

<template>
  <div class="container">
    <div class="row">
      <button class="btn" style="width: 150px; height: 150px; margin-right: 100px" @click="setFanSpeed(0)">Off</button>
      <button class="btn" style="width: 150px; height: 150px; margin-right: 100px" @click="setFanSpeed(1)">Low</button>
      <button class="btn" style="width: 150px; height: 150px; margin-right: 100px" @click="setFanSpeed(2)">Med</button>
      <button class="btn" style="width: 150px; height: 150px; margin-right: 100px" @click="setFanSpeed(3)">High</button>
    </div>
    <Temperature />
  </div>
</template>

<style scoped>

</style>
