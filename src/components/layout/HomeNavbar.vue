<script setup lang="ts">
import { PhClockCounterClockwise, PhFaders } from "@phosphor-icons/vue";

const displayCompleted = defineModel<boolean>({
  required: true,
});
</script>

<template>
  <div class="drawer">
    <input id="my-drawer" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content flex flex-col">
      <!-- Navbar -->
      <div class="navbar bg-base-100">
        <div class="navbar-start">
          <label
            for="my-drawer"
            aria-label="open sidebar"
            class="btn btn-square btn-ghost"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 6h16M4 12h16M4 18h7"
              />
            </svg>
          </label>
        </div>
        <label class="navbar-center swap">
          <input type="checkbox" v-model="displayCompleted" />
          <div class="swap-off text-center text-xl font-bold">未完了</div>
          <div class="swap-on text-center text-xl font-bold">完了</div>
        </label>
        <div class="navbar-end">
          <button class="btn btn-circle btn-ghost">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
              />
            </svg>
          </button>
          <button class="btn btn-circle btn-ghost">
            <div class="indicator">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-5 w-5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
                />
              </svg>
              <span class="badge indicator-item badge-primary badge-xs"></span>
            </div>
          </button>
        </div>
      </div>
      <!-- Page content here -->
      <slot />
    </div>

    <div class="drawer-side">
      <label
        for="my-drawer"
        aria-label="close sidebar"
        class="drawer-overlay"
      ></label>
      <ul class="menu min-h-full w-80 bg-base-200 p-4">
        <!-- Sidebar content here -->
        <li>
          <RouterLink :to="{ name: 'settings' }">
            <div class="flex flex-row items-center gap-2 text-lg">
              <PhFaders /> 設定
            </div></RouterLink
          >
        </li>
        <li>
          <RouterLink :to="{ name: 'coming_soon' }">
            <div class="flex flex-row items-center gap-2 text-lg">
              <PhClockCounterClockwise /> タイムライン
            </div>
          </RouterLink>
        </li>
      </ul>
    </div>
  </div>
</template>
