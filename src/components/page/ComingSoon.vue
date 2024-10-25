<template>
  <div class="flex h-screen items-center justify-center">
    <div class="flex flex-col gap-1">
      <div class="flex animate-pulse flex-col">
        <h1 class="mb-4 text-center text-5xl font-bold">Coming Soon</h1>
        <p class="text-xl">鋭意制作中です。もうしばらくお待ちください。</p>
      </div>
      <div class="flex justify-end px-5">
        <button class="btn btn-ghost" @click="$router.back()">戻る</button>
      </div>
    </div>
  </div>
</template>
