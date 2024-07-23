<script setup lang="ts">
import LineChart from './components/LineChart.vue'
import { ref } from 'vue'

const url = ref('ws://127.0.0.1:8000/ws')
</script>

<template>
  <div class="layout">
    <el-container>
      <el-header class="header">
        <el-row :gutter="20">
          <el-col :span="16">
            <h1>Real-time Line Chart(WebSocket Test)</h1>
          </el-col>
          <el-col :span="8" class="url-col">
            <el-row :gutter="20" style="width: 100%">
              <el-col :span="4">
                <el-text size="large">URL:</el-text>
              </el-col>
              <el-col :span="20">
                <el-input v-model="url" style="width: 100%" placeholder="WebSocket URL" />
              </el-col>
            </el-row>
          </el-col>
        </el-row>
      </el-header>
      <el-main class="main">
        <LineChart :url="url" />
      </el-main>
      <el-footer class="footer">
        <el-text size="small">Powered by Chart.JS & vue-chartjs</el-text>
        <br>
        <el-text size="small">Author:
          <el-link type="primary" href="https://github.com/Akiko97">Akiko97</el-link>
        </el-text>
      </el-footer>
    </el-container>
  </div>
</template>

<style scoped>
.layout {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
}
.header, .footer {
  flex-shrink: 0;
}
.main {
  flex-grow: 1;
  overflow: auto;
}
.url-col {
  display: flex;
  align-items: center;
}
</style>
