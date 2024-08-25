<script lang="ts" setup>
import { useRoute } from "vue-router";

const route = useRoute();
</script>

<template>
  <div class="h-screen select-none overflow-hidden p-5 font-mono">
    <RouterView :key="route.fullPath" class="h-full w-full" />
  </div>
</template>
