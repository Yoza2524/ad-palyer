<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
// ------------------- ^待删除^ -------------------

const songs = ref([
  // 这里我们将通过 Tauri 的 API 来获取本地文件，目前先用模拟数据
  { name: '示例歌曲 1.mp3', path: '/path/to/your/song1.mp3' },
  { name: '示例歌曲 2.mp3', path: '/path/to/your/song2.mp3' },
]);

const currentSong = ref(null);
const audioPlayer = ref(null);
const playSong = (song) => {
  currentSong.value = song;
  // TODO: 在这里我们将使用 Tauri API 来获取歌曲的实际路径
  // 目前，我们假定路径已经可用
  audioPlayer.value.src = song.path;
  audioPlayer.value.play();
};

</script>

<template>
   <main class="container">
    <h1>Ad-player</h1>
    
    <div class="song-list">
      <ul>
        <li v-for="(song, index) in songs" :key="index">
          {{ song.name }}
          <button @click="playSong(song)">播放</button>
        </li>
      </ul>
    </div>

    <div class="player-controls">
      <p>正在播放: {{ currentSong ? currentSong.name : '无' }}</p>
      <audio ref="audioPlayer" controls></audio>
    </div>
  </main>
</template>
