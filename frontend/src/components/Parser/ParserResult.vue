<template>
  <transition name="result-appear" mode="out-in">
    <v-card 
      v-if="appStore.hasResult && result" 
      class="result-card"
      :class="themeClass"
      elevation="16"
    >
      <v-card-text class="pa-8">
        <!-- 标题栏 -->
        <div class="result-header mb-6 animate__animated animate__fadeInDown">
          <v-icon icon="mdi-check-circle" size="32" color="success" class="mr-3"></v-icon>
          <h2 class="text-h5 font-weight-bold">解析结果</h2>
        </div>

        <!-- 视频播放器区域 -->
        <div v-if="result.video_url" class="video-player-section mb-6 animate__animated animate__fadeIn">
          <v-card class="player-card" elevation="8">
            <v-card-title class="d-flex align-center justify-space-between pa-4">
              <div class="d-flex align-center">
                <v-icon icon="mdi-play-circle" color="primary" size="28" class="mr-2"></v-icon>
                <span class="text-h6">视频预览</span>
              </div>
              <v-chip color="success" size="small" variant="flat">
                <v-icon icon="mdi-check" start size="16"></v-icon>
                可播放
              </v-chip>
            </v-card-title>

            <v-divider></v-divider>

            <!-- 视频播放器 -->
            <div class="video-container">
              <video
                ref="videoPlayer"
                class="video-element"
                :src="proxyVideoUrl"
                :poster="result.cover"
                controls
                controlslist="nodownload"
                preload="metadata"
                @loadstart="handleVideoLoadStart"
                @canplay="handleVideoCanPlay"
                @error="handleVideoError"
              >
                您的浏览器不支持视频播放。
              </video>

              <!-- 加载遮罩 -->
              <transition name="fade">
                <div v-if="videoLoading" class="video-loading-overlay">
                  <v-progress-circular
                    indeterminate
                    color="primary"
                    size="64"
                    width="6"
                  ></v-progress-circular>
                  <p class="mt-4 text-h6">加载视频中...</p>
                </div>
              </transition>

              <!-- 错误遮罩 -->
              <transition name="fade">
                <div v-if="videoError" class="video-error-overlay">
                  <v-icon icon="mdi-alert-circle" size="64" color="error"></v-icon>
                  <p class="mt-4 text-h6">视频加载失败</p>
                  <p class="text-body-2 text-grey mt-2">请尝试直接下载观看</p>
                </div>
              </transition>
            </div>

            <!-- 播放器控制栏 -->
            <v-card-actions class="pa-4">
              <v-btn
                color="primary"
                variant="flat"
                prepend-icon="mdi-play"
                @click="playVideo"
                :disabled="videoError"
              >
                播放
              </v-btn>

              <v-btn
                color="secondary"
                variant="flat"
                prepend-icon="mdi-pause"
                @click="pauseVideo"
                :disabled="videoError"
              >
                暂停
              </v-btn>

              <v-spacer></v-spacer>

              <v-btn
                color="success"
                variant="flat"
                prepend-icon="mdi-download"
                @click="downloadVideo"
                size="large"
              >
                下载视频
              </v-btn>

              <v-btn
                color="info"
                variant="outlined"
                prepend-icon="mdi-open-in-new"
                :href="result.video_url"
                target="_blank"
              >
                新窗口打开
              </v-btn>
            </v-card-actions>
          </v-card>
        </div>

        <v-row>
          <!-- 左侧：视频信息 -->
          <v-col cols="12" md="8">
            <div class="video-info animate__animated animate__fadeInLeft">
              <!-- 标题 -->
              <div v-if="result.title" class="info-item mb-4">
                <v-chip
                  color="primary"
                  variant="flat"
                  size="small"
                  class="mb-2"
                  prepend-icon="mdi-format-title"
                >
                  标题
                </v-chip>
                <p class="text-h6 font-weight-medium">{{ result.title }}</p>
              </div>

              <!-- 作者 -->
              <div v-if="result.author" class="info-item mb-4">
                <v-chip
                  color="secondary"
                  variant="flat"
                  size="small"
                  class="mb-2"
                  prepend-icon="mdi-account"
                >
                  作者
                </v-chip>
                <p class="text-body-1">{{ result.author }}</p>
              </div>

              <!-- 平台 -->
              <div v-if="result.platform" class="info-item mb-4">
                <v-chip
                  color="accent"
                  variant="flat"
                  size="small"
                  class="mb-2"
                  prepend-icon="mdi-platform"
                >
                  平台
                </v-chip>
                <p class="text-body-1">{{ result.platform }}</p>
              </div>

              <!-- 描述 -->
              <div v-if="result.description" class="info-item mb-4">
                <v-chip
                  color="info"
                  variant="flat"
                  size="small"
                  class="mb-2"
                  prepend-icon="mdi-text"
                >
                  描述
                </v-chip>
                <p class="text-body-2">{{ result.description }}</p>
              </div>
            </div>
          </v-col>

          <!-- 右侧：封面图 -->
          <v-col cols="12" md="4">
            <div 
              v-if="result.cover" 
              class="cover-container animate__animated animate__fadeInRight"
            >
              <v-card elevation="4">
                <v-img
                  :src="result.cover"
                  aspect-ratio="1"
                  cover
                  class="cover-image"
                >
                  <template v-slot:placeholder>
                    <v-row
                      class="fill-height ma-0"
                      align="center"
                      justify="center"
                    >
                      <v-progress-circular
                        indeterminate
                        color="primary"
                      ></v-progress-circular>
                    </v-row>
                  </template>
                </v-img>
                <div class="cover-overlay">
                  <v-icon icon="mdi-image" size="64" color="white" class="overlay-icon"></v-icon>
                </div>
                
                <!-- 封面下载按钮 -->
                <v-card-actions class="pa-2">
                  <v-btn
                    color="success"
                    variant="flat"
                    prepend-icon="mdi-download"
                    size="small"
                    block
                    @click="downloadCoverImage"
                  >
                    下载封面
                  </v-btn>
                </v-card-actions>
              </v-card>
            </div>
            
            <!-- 无封面时的占位符 -->
            <div v-else class="no-cover animate__animated animate__fadeInRight">
              <v-icon icon="mdi-image-off" size="64" color="grey"></v-icon>
              <p class="text-body-2 text-grey mt-2">暂无封面</p>
            </div>
          </v-col>
        </v-row>

        <!-- 图片展示和下载区域 -->
        <div v-if="hasImages" class="images-section mt-8 animate__animated animate__fadeInUp">
          <v-card class="images-card" elevation="8">
            <v-card-title class="d-flex align-center justify-space-between pa-4">
              <div class="d-flex align-center">
                <v-icon icon="mdi-image-multiple" color="primary" size="28" class="mr-2"></v-icon>
                <span class="text-h6">图片集 ({{ result.images?.length || 0 }})</span>
              </div>
              <div class="d-flex gap-2">
                <v-btn
                  color="primary"
                  variant="flat"
                  prepend-icon="mdi-check-all"
                  @click="selectAllImages"
                  size="small"
                >
                  全选
                </v-btn>
                <v-btn
                  color="secondary"
                  variant="outlined"
                  prepend-icon="mdi-close"
                  @click="clearSelection"
                  size="small"
                  :disabled="selectedImages.size === 0"
                >
                  清除选择
                </v-btn>
                <v-btn
                  color="success"
                  variant="flat"
                  prepend-icon="mdi-download-multiple"
                  @click="downloadSelectedImages"
                  size="small"
                  :disabled="selectedImages.size === 0"
                >
                  下载选中 ({{ selectedImages.size }})
                </v-btn>
              </div>
            </v-card-title>

            <v-divider></v-divider>

            <v-card-text class="pa-4">
              <v-row>
                <v-col
                  v-for="(image, index) in result.images"
                  :key="index"
                  cols="12"
                  sm="6"
                  md="4"
                  lg="3"
                >
                  <v-card
                    class="image-card"
                    :class="{ 'image-selected': selectedImages.has(index) }"
                    elevation="4"
                    @click="toggleImageSelection(index)"
                  >
                    <v-img
                      :src="image.url"
                      aspect-ratio="1"
                      cover
                      class="image-preview"
                    >
                      <template v-slot:placeholder>
                        <v-row
                          class="fill-height ma-0"
                          align="center"
                          justify="center"
                        >
                          <v-progress-circular
                            indeterminate
                            color="primary"
                          ></v-progress-circular>
                        </v-row>
                      </template>

                      <!-- 选中标记 -->
                      <div class="image-overlay">
                        <v-checkbox
                          :model-value="selectedImages.has(index)"
                          color="primary"
                          hide-details
                          class="selection-checkbox"
                          @click.stop
                        ></v-checkbox>
                        <div class="image-number">{{ index + 1 }}</div>
                      </div>
                    </v-img>

                    <v-card-actions class="pa-2">
                      <v-btn
                        color="success"
                        variant="flat"
                        prepend-icon="mdi-download"
                        size="small"
                        block
                        @click.stop="downloadSingleImage(image, index)"
                      >
                        下载
                      </v-btn>
                    </v-card-actions>
                  </v-card>
                </v-col>
              </v-row>
            </v-card-text>
          </v-card>
        </div>
      </v-card-text>
    </v-card>
  </transition>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAppStore, type ImgInfo } from '@/stores/app'
import { useThemeStore } from '@/stores/theme'
import { useToast } from '@/composables/useToast'
import { downloadFile, generateSafeFilename, getFileExtension } from '@/utils/download'

const appStore = useAppStore()
const themeStore = useThemeStore()
const { showToast } = useToast()

const videoPlayer = ref<HTMLVideoElement | null>(null)
const videoLoading = ref(false)
const videoError = ref(false)
const selectedImages = ref<Set<number>>(new Set())

const result = computed(() => appStore.result)
const themeClass = computed(() => `theme-${themeStore.currentTheme}`)
const hasImages = computed(() => result.value?.images && result.value.images.length > 0)

// 使用代理URL避免跨域问题
const proxyVideoUrl = computed(() => {
  if (!result.value?.video_url) return ''
  return `/api/proxy/video?url=${encodeURIComponent(result.value.video_url)}`
})

function handleVideoLoadStart() {
  videoLoading.value = true
  videoError.value = false
}

function handleVideoCanPlay() {
  videoLoading.value = false
  videoError.value = false
}

function handleVideoError() {
  videoLoading.value = false
  videoError.value = true
  showToast('视频加载失败，请尝试下载后观看', 'error')
}

function playVideo() {
  if (videoPlayer.value) {
    videoPlayer.value.play()
    showToast('开始播放', 'info')
  }
}

function pauseVideo() {
  if (videoPlayer.value) {
    videoPlayer.value.pause()
    showToast('暂停播放', 'info')
  }
}

async function downloadVideo() {
  if (!result.value?.video_url) return

  try {
    showToast('准备下载视频...', 'info')
    
    const extension = getFileExtension(result.value.video_url)
    const filename = generateSafeFilename(result.value.title || 'video', extension)
    
    await downloadFile(proxyVideoUrl.value, filename)
    
    showToast('下载已开始，请查看浏览器下载项', 'success')
  } catch (error) {
    console.error('Download error:', error)
    showToast('下载失败，您可以尝试右键点击视频另存为', 'error')
  }
}

// 图片选择功能
function toggleImageSelection(index: number) {
  if (selectedImages.value.has(index)) {
    selectedImages.value.delete(index)
  } else {
    selectedImages.value.add(index)
  }
}

function selectAllImages() {
  if (!result.value?.images) return
  selectedImages.value = new Set(result.value.images.map((_, index) => index))
  showToast(`已选择全部 ${selectedImages.value.size} 张图片`, 'success')
}

function clearSelection() {
  selectedImages.value.clear()
  showToast('已清除选择', 'info')
}

// 下载单张图片
async function downloadSingleImage(image: ImgInfo, index: number) {
  try {
    showToast(`开始下载图片 ${index + 1}...`, 'info')
    
    const proxyUrl = `/api/proxy/image?url=${encodeURIComponent(image.url)}`
    const extension = getFileExtension(image.url)
    const filename = `image_${index + 1}.${extension}`
    
    await downloadFile(proxyUrl, filename)
    showToast(`图片 ${index + 1} 下载成功`, 'success')
  } catch (error) {
    console.error('Image download error:', error)
    showToast(`图片 ${index + 1} 下载失败`, 'error')
  }
}

// 下载封面图片
async function downloadCoverImage() {
  if (!result.value?.cover) return

  try {
    showToast('开始下载封面...', 'info')
    
    const proxyUrl = `/api/proxy/image?url=${encodeURIComponent(result.value.cover)}`
    const extension = getFileExtension(result.value.cover)
    const filename = `cover.${extension}`
    
    await downloadFile(proxyUrl, filename)
    showToast('封面下载成功', 'success')
  } catch (error) {
    console.error('Cover download error:', error)
    showToast('封面下载失败', 'error')
  }
}

// 批量下载选中的图片
async function downloadSelectedImages() {
  if (selectedImages.value.size === 0) {
    showToast('请先选择要下载的图片', 'warning')
    return
  }

  if (!result.value?.images) return

  const total = selectedImages.value.size
  let success = 0
  let failed = 0

  showToast(`开始批量下载 ${total} 张图片...`, 'info')

  // 批量下载，添加延迟避免请求过快
  for (const index of Array.from(selectedImages.value)) {
    const image = result.value.images[index]
    if (!image) continue

    try {
      const proxyUrl = `/api/proxy/image?url=${encodeURIComponent(image.url)}`
      const extension = getFileExtension(image.url)
      const filename = `image_${index + 1}.${extension}`
      
      await downloadFile(proxyUrl, filename)
      success++
      
      // 添加短暂延迟，避免请求过快
      await new Promise(resolve => setTimeout(resolve, 500))
    } catch (error) {
      console.error(`Image ${index + 1} download error:`, error)
      failed++
    }
  }

  if (failed === 0) {
    showToast(`批量下载完成！成功 ${success} 张`, 'success')
  } else {
    showToast(`下载完成：成功 ${success} 张，失败 ${failed} 张`, 'warning')
  }

  // 下载完成后清除选择
  clearSelection()
}
</script>

<style scoped lang="scss">
.result-card {
  position: relative;
  overflow: hidden;
  background: rgba(var(--v-theme-surface), 0.95);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(var(--v-theme-success), 0.3);
  transition: all 0.4s ease;

  &:hover {
    border-color: rgba(var(--v-theme-success), 0.6);
    transform: translateY(-4px);
    box-shadow: 0 16px 48px rgba(var(--v-theme-success), 0.3);
  }

  &.theme-cyber,
  &.theme-neon {
    &::after {
      content: '';
      position: absolute;
      top: 0;
      left: -100%;
      width: 100%;
      height: 100%;
      background: linear-gradient(90deg, 
        transparent,
        rgba(var(--v-theme-success), 0.2),
        transparent
      );
      animation: shimmer 3s infinite;
    }
  }

  &.theme-glassmorphism {
    background: rgba(var(--v-theme-surface), 0.4);
    backdrop-filter: blur(30px);
  }
}

.result-header {
  display: flex;
  align-items: center;
}

.video-player-section {
  .player-card {
    border: 2px solid rgba(var(--v-theme-primary), 0.3);
    transition: all 0.3s ease;

    &:hover {
      border-color: rgba(var(--v-theme-primary), 0.6);
      box-shadow: 0 8px 32px rgba(var(--v-theme-primary), 0.3);
    }
  }
}

.video-container {
  position: relative;
  width: 100%;
  background: #000;
  min-height: 400px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.video-element {
  width: 100%;
  max-height: 600px;
  display: block;
  background: #000;
}

.video-loading-overlay,
.video-error-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(10px);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  z-index: 10;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.video-info {
  .info-item {
    padding: 16px;
    border-radius: 12px;
    background: rgba(var(--v-theme-background), 0.3);
    transition: all 0.3s ease;

    &:hover {
      background: rgba(var(--v-theme-background), 0.5);
      transform: translateX(8px);
    }
  }
}

.cover-container {
  position: relative;
  
  .v-card {
    border-radius: 16px;
    overflow: hidden;
    transition: all 0.3s ease;
    border: 2px solid rgba(var(--v-theme-primary), 0.2);

    &:hover {
      transform: translateY(-4px);
      box-shadow: 0 12px 48px rgba(var(--v-theme-primary), 0.4);
      border-color: rgba(var(--v-theme-primary), 0.5);

      .cover-overlay {
        opacity: 1;
      }
    }
  }
}

.cover-image {
  position: relative;
}

.cover-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(var(--v-theme-primary), 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.3s ease;
  backdrop-filter: blur(5px);
  pointer-events: none;
}

.overlay-icon {
  animation: iconBounce 1s ease infinite;
}

.no-cover {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  min-height: 300px;
  border: 2px dashed rgba(var(--v-theme-on-surface), 0.3);
  border-radius: 16px;
  background: rgba(var(--v-theme-background), 0.3);
}

// 图片区域样式
.images-section {
  .images-card {
    border: 2px solid rgba(var(--v-theme-info), 0.3);
    transition: all 0.3s ease;

    &:hover {
      border-color: rgba(var(--v-theme-info), 0.6);
      box-shadow: 0 8px 32px rgba(var(--v-theme-info), 0.3);
    }
  }
}

.image-card {
  cursor: pointer;
  transition: all 0.3s ease;
  border: 2px solid transparent;
  position: relative;

  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 24px rgba(var(--v-theme-primary), 0.3);

    .image-overlay {
      opacity: 1;
    }
  }

  &.image-selected {
    border-color: rgba(var(--v-theme-primary), 0.8);
    box-shadow: 0 4px 16px rgba(var(--v-theme-primary), 0.4);
  }
}

.image-preview {
  position: relative;
}

.image-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    to bottom,
    rgba(0, 0, 0, 0.6) 0%,
    transparent 30%,
    transparent 70%,
    rgba(0, 0, 0, 0.6) 100%
  );
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 8px;
  opacity: 0;
  transition: opacity 0.3s ease;

  .image-card:hover &,
  .image-card.image-selected & {
    opacity: 1;
  }
}

.selection-checkbox {
  background: rgba(255, 255, 255, 0.9);
  border-radius: 8px;
  padding: 4px;
}

.image-number {
  background: rgba(var(--v-theme-primary), 0.9);
  color: white;
  padding: 4px 12px;
  border-radius: 12px;
  font-weight: bold;
  font-size: 14px;
}

// 结果出现动画
.result-appear-enter-active {
  animation: resultSlideIn 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

.result-appear-leave-active {
  animation: resultSlideOut 0.4s ease;
}

@keyframes resultSlideIn {
  from {
    opacity: 0;
    transform: translateY(50px) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

@keyframes resultSlideOut {
  from {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
  to {
    opacity: 0;
    transform: translateY(-50px) scale(0.9);
  }
}

@keyframes shimmer {
  to {
    left: 100%;
  }
}

@keyframes iconBounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}

// 响应式调整
@media (max-width: 960px) {
  .video-container {
    min-height: 300px;
  }

  .video-element {
    max-height: 400px;
  }

  .images-card {
    .v-card-title {
      flex-direction: column;
      gap: 12px;

      > div {
        width: 100%;
        justify-content: center;
      }
    }
  }
}

@media (max-width: 600px) {
  .video-container {
    min-height: 200px;
  }

  .video-element {
    max-height: 300px;
  }

  .player-card {
    .v-card-actions {
      flex-direction: column;
      gap: 8px;

      .v-btn {
        width: 100%;
      }

      .v-spacer {
        display: none;
      }
    }
  }

  .images-card {
    .v-card-title {
      .d-flex.gap-2 {
        flex-direction: column;
        width: 100%;

        .v-btn {
          width: 100%;
        }
      }
    }
  }
}
</style>
