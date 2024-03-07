<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import {invoke} from "@tauri-apps/api/tauri";

async function turnOn() {
  console.log('turning on');
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  invoke("turn_on");
}

async function turnOff() {
  console.log('turning off');
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("turn_off");
}
</script>

<template>
  <div class="container">
    <div class="row">
      <button class="btn" style="width: 300px; height: 300px; margin-right: 100px" @click="turnOn">On</button>
      <button class="btn" style="width: 300px; height: 300px" @click="turnOff">Off</button>
    </div>
<!--    <Greet />-->
  </div>
</template>

<style scoped>

</style>
