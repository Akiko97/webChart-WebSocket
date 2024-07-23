<script setup lang="ts">
const props = defineProps<{ url: string }>()

import {ref} from 'vue'
import {CategoryScale, Chart as ChartJS, Legend, LinearScale, LineElement, PointElement, Title, Tooltip} from 'chart.js'
import {Line} from 'vue-chartjs'

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
)

const data = ref<{
  labels: string[],
  datasets: {
    label: string,
    backgroundColor: string,
    data: number[],
  }[]
}>({
  labels: [],
  datasets: [
    {
      label: 'Data Received',
      backgroundColor: '#f87979',
      data: []
    }
  ]
})

const options = {
  responsive: true,
  maintainAspectRatio: false
}

const data_no = ref(0)

const connectWebSocket = () => {
  const ws = new WebSocket(props.url)

  ws.onmessage = (event) => {
    const num: number = event.data
    data.value = {
      labels: [...data.value.labels, `Data${data_no.value++}`],
      datasets: [
        {
          ...data.value.datasets[0],
          data: [...data.value.datasets[0].data, num]
        }
      ]
    }
  }
}
const connect = () => {
  connectWebSocket()
}
const clear = () => {
  data.value = {
    labels: [],
    datasets: [
      {
        label: 'Data Received',
        backgroundColor: '#f87979',
        data: []
      }
    ]
  }
  data_no.value = 0
}
</script>

<template>
  <el-container class="line-chart-container">
    <el-aside width="20%" class="side-bar">
      Control
      <el-divider />
      <el-space direction="vertical">
        <el-button type="primary" text bg color="#E5EAF3" @click="connect">Connect</el-button>
        <el-button type="primary" text bg color="#E5EAF3" @click="clear">Clear</el-button>
      </el-space>
    </el-aside>
    <el-main>
      <Line :data="data" :options="options" />
    </el-main>
  </el-container>
</template>

<style scoped>
.line-chart-container {
  height: 100%;
  background-color: #E5EAF3;
  border-radius: 5px;
}
.side-bar {
  height: 100%;
  background-color: #CFD3DC;
  border-radius: 5px;
}
</style>
