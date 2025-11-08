import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface Author {
  uid: string
  name: string
  avatar: string
}

export interface ImgInfo {
  url: string
  live_photo_url?: string
}

export interface VideoParseInfo {
  author: Author
  title: string
  video_url?: string
  music_url?: string
  cover_url?: string
  images?: ImgInfo[]
}

export interface ParseResult {
  title: string
  author: string
  cover?: string
  video_url?: string
  description?: string
  platform?: string
  images?: ImgInfo[]
  music_url?: string
}

export const useAppStore = defineStore('app', () => {
  const inputUrl = ref('')
  const loading = ref(false)
  const result = ref<ParseResult | null>(null)
  const error = ref<string | null>(null)

  const hasResult = computed(() => result.value !== null)
  const hasError = computed(() => error.value !== null)

  async function parseUrl() {
    if (!inputUrl.value.trim()) {
      error.value = '请输入视频链接'
      return
    }

    loading.value = true
    error.value = null
    result.value = null

    try {
      // 使用后端的GET接口: /video/share/url/parse?url=xxx
      const encodedUrl = encodeURIComponent(inputUrl.value)
      const response = await fetch(`/api/video/share/url/parse?url=${encodedUrl}`)

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }

      const data: { code: number; msg: string; data?: VideoParseInfo } = await response.json()

      if (data.code !== 200) {
        error.value = data.msg || '解析失败'
      } else if (data.data) {
        // 转换后端数据格式为前端格式
        const videoInfo = data.data
        result.value = {
          title: videoInfo.title,
          author: videoInfo.author.name,
          cover: videoInfo.cover_url,
          video_url: videoInfo.video_url,
          music_url: videoInfo.music_url,
          images: videoInfo.images || [],
          description: videoInfo.images && videoInfo.images.length > 0 
            ? `包含 ${videoInfo.images.length} 张图片` 
            : undefined
        }
      } else {
        error.value = '解析失败，未获取到数据'
      }
    } catch (e) {
      console.error('Parse error:', e)
      error.value = '解析失败，请检查网络连接或后端服务是否运行'
    } finally {
      loading.value = false
    }
  }

  function clearResult() {
    inputUrl.value = ''
    result.value = null
    error.value = null
  }

  return {
    inputUrl,
    loading,
    result,
    error,
    hasResult,
    hasError,
    parseUrl,
    clearResult
  }
})
