<script setup>
import { invoke } from "@tauri-apps/api/core";

// const playPauseBtn = document.getElementById('play-pause-btn');
// const selectFileBtn = document.getElementById('select-file-btn');
// const stopBtn = document.getElementById('stop-btn');

// let isPlaying = false;
// let currentPath = null;

function playAduio() {
  invoke("play_audio", { path: ""})
}

</script>

<template>
  <main>
    <button><<</button>
    <button @click="playAduio">II</button>
    <button>>></button>
  </main>
</template>
