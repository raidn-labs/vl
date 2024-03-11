<script setup>
import BlowerControl from "./components/BlowerControl.vue";
import UtilityBar from "./components/UtilityBar.vue";
import Navigation from "./components/Navigation.vue";
</script>

<template>
  <div class="flex flex-col h-screen bg-gray-800">
    <!-- Utility Bar -->
    <UtilityBar />

    <!-- Main Content -->
    <div class="flex-1 overflow-y-auto">
      <BlowerControl />
    </div>

    <!-- Sticky Navigation Bar -->
    <Navigation />
  </div>
</template>

<style scoped>

</style>
