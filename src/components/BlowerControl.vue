<script setup>
import {invoke} from "@tauri-apps/api/tauri";

async function setFanSpeed(speed) {
  await invoke("set_fan_speed", { speed });
}
</script>

<template>
  <div class="flex">
      <button type="button" class="mr-4 rounded-md bg-white/10 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-white/20" @click="setFanSpeed(0)">Off</button>
      <button type="button" class="mr-4 rounded-md bg-white/10 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-white/20" @click="setFanSpeed(1)">Low</button>
      <button type="button" class="mr-4 rounded-md bg-white/10 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-white/20" @click="setFanSpeed(2)">Med</button>
      <button type="button" class="mr-4 rounded-md bg-white/10 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-white/20" @click="setFanSpeed(3)">High</button>
  </div>
</template>