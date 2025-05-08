<template>
  <div class="container" data-tauri-drag-region @mousedown="startDragging">
    <n-dropdown trigger="hover" placement="top-end" :options="options" @select="handleSelect">
      <!-- <n-button type="success">找个地方休息</n-button> -->
      <h2 class="clock">{{ currentTime }}</h2>
    </n-dropdown>
    <!-- <Greet /> -->
  </div>
</template>


<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
import { ref, onMounted, onBeforeUnmount } from 'vue';
import { appWindow } from '@tauri-apps/api/window';
// import { useSound } from '@vueuse/sound'
// import s01 from '@/assets/end.aac' // 参考音色
// import s01 from './assets/end.aac' // 参考音色
// const aa = useSound(url)
// aa.play() // 无效
// 时钟模式 - 'clock'表示实时时间，'timer'表示倒计时
const clockMode = ref('clock');
// 倒计时剩余时间（秒）
const remainingSeconds = ref(0);
// 倒计时目标时间
const targetTime = ref(null);

// 创建响应式的时间变量
const currentTime = ref('');
let timer = null;

const options = ref([
  {
    label: '当前时间',
    key: 'now',
  },
  {
    label: '25分钟',
    key: '25',
  },
  {
    label: '15分钟',
    key: '15',
  },
  {
    label: '5分钟',
    key: '5',
  },
]);




// 格式化倒计时
function formatCountdown(seconds) {
  const mins = Math.floor(seconds / 60).toString().padStart(2, '0');
  const secs = (seconds % 60).toString().padStart(2, '0');
  return `${mins}:${secs}`;
}

// 更新时间的函数
function updateTime() {
  if (clockMode.value === 'clock') {
    // 实时时钟模式
    currentTime.value = formatTime(new Date());
  } else {
    // 倒计时模式
    const now = new Date().getTime();
    const target = targetTime.value.getTime();
    const diff = Math.max(0, Math.floor((target - now) / 1000));

    remainingSeconds.value = diff;

    if (diff <= 0) {
      // 倒计时结束
      notifyCountdownEnd();
      resetToClockMode();
    } else {
      currentTime.value = formatCountdown(diff);
    }
  }
}

// 设置倒计时
function startCountdown(minutes) {
  clockMode.value = 'timer';
  const now = new Date();
  targetTime.value = new Date(now.getTime() + minutes * 60 * 1000);
  remainingSeconds.value = minutes / 60;
  updateTime(); // 立即更新显示
}

// 播放结束提示音
function playEndSound0() {
  try {
    // 创建音频对象
    const audio = new Audio('@assets/end.aac');

    // 设置音频参数
    audio.volume = 1.0; // 音量设置为最大

    // 播放音频
    audio.play().catch(error => {
      console.error('音频播放失败:', error);
    });

    // 可选：在音频播放结束后释放资源
    audio.onended = () => {
      audio.src = '';
    };
  } catch (error) {
    console.error('创建音频对象失败:', error);
  }
}


function playEndSound() {
  // 创建 AudioContext
  const audioContext = new (window.AudioContext || window.webkitAudioContext)();

  // 创建振荡器
  const oscillator = audioContext.createOscillator();
  const gainNode = audioContext.createGain();

  // 设置音色
  oscillator.type = 'sine'; // 正弦波
  oscillator.frequency.value = 880; // 音高

  // 设置音量
  gainNode.gain.value = 0.5;

  // 连接节点
  oscillator.connect(gainNode);
  gainNode.connect(audioContext.destination);

  // 播放声音
  oscillator.start();

  // 0.5秒后停止
  setTimeout(() => {
    oscillator.stop();
  }, 500);
}
// 倒计时结束通知
function notifyCountdownEnd() {
  // 可以在这里添加通知逻辑
  console.log('倒计时结束！');
  // 播放音频提示
  playEndSound();
  resetToClockMode();
}
// 重置为时钟模式
function resetToClockMode() {
  clockMode.value = 'clock';
  targetTime.value = null;
  remainingSeconds.value = 0;
  updateTime(); // 立即更新显示
}

// 格式化时间的函数
function formatTime(date) {
  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  const seconds = date.getSeconds().toString().padStart(2, '0');
  return `${hours}:${minutes}:${seconds}`;
}

// // 更新时间的函数
// function updateTime() {
//   currentTime.value = formatTime(new Date());
// }

// 窗口拖拽函数
async function startDragging(event) {
  // 防止其他鼠标事件触发
  event.preventDefault();

  try {
    // 调用 Tauri API 开始拖拽
    await appWindow.startDragging();
  } catch (error) {
    console.error('拖拽失败:', error);
  }
}

// 处理下拉菜单选择
const handleSelect = (key) => {
  switch (key) {
    case 'now':
      resetToClockMode();
      break;
    case '25':
      startCountdown(25);
      break;
    case '15':
      startCountdown(15);
      break;
    case '5':
      startCountdown(5);
      break;
    default:
      break;
  }
};

// 在组件挂载时启动定时器
onMounted(() => {
  // playEndSound();
  // 立即更新一次时间
  updateTime();

  // 设置每秒更新一次的定时器
  timer = setInterval(updateTime, 1000);
});

// 在组件卸载前清除定时器
onBeforeUnmount(() => {
  if (timer) {
    clearInterval(timer);
    timer = null;
  }
});
</script>


<style scoped>
.clock {
  font-size: 1.75rem;
  font-weight: 700;
  color: #ffffffde;
  /* text-shadow: 0 0 10px rgba(0, 0, 0, 0.5); */
  text-shadow: 0 0 10px rgba(1, 1, 1, 0.5);
  /* margin: 20px 0; */
  font-family: 'Courier New', monospace;
  background: rgba(0, 0, 0, 0.3);
  padding: 0.2rem 1rem;
  display: inline-block;
  border-radius: 1.1rem;
  margin: 0;
}

.container {
  position: absolute;
  bottom: 0;
  left: 0;
  background-color: transparent;
}
</style>