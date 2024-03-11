<script setup>
import BlowerControl from "./components/BlowerControl.vue";
import UtilityBar from "./components/UtilityBar.vue";
import Navigation from "./components/Navigation.vue";
import {ref} from 'vue';

const activeScreen = ref('home');

function isActiveScreen (screen) {
  return activeScreen.value === screen;
}
</script>

<template>
  <div class="flex flex-col h-screen bg-gray-800">
    <!-- Utility Bar -->
    <UtilityBar />

    <!-- Main Content -->
    <div class="flex-1 overflow-y-auto p-20">
      <div v-if="isActiveScreen('home')">
        <BlowerControl />
      </div>
      <div v-if="isActiveScreen('music')">
        <h1 class="text-white">Music</h1>
      </div>
      <div v-if="isActiveScreen('settings')">
        <h1 class="text-white">Settings</h1>
      </div>
    </div>

    <!-- Sticky Navigation Bar -->
    <Navigation @navigate="(to) => activeScreen = to"/>
  </div>
</template>
