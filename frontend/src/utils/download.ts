/**
 * 下载工具函数
 */

/**
 * 下载文件
 * @param url 文件URL
 * @param filename 保存的文件名
 */
export async function downloadFile(url: string, filename: string): Promise<void> {
  try {
    // 方法1: 使用fetch下载（支持跨域代理）
    const response = await fetch(url)
    const blob = await response.blob()
    const objectUrl = URL.createObjectURL(blob)
    
    const link = document.createElement('a')
    link.href = objectUrl
    link.download = filename
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    
    // 释放对象URL
    setTimeout(() => URL.revokeObjectURL(objectUrl), 100)
  } catch (error) {
    // 方法2: 直接使用链接（可能会被拦截）
    const link = document.createElement('a')
    link.href = url
    link.download = filename
    link.target = '_blank'
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }
}

/**
 * 下载视频（带进度支持）
 * @param url 视频URL
 * @param filename 文件名
 * @param onProgress 进度回调
 */
export async function downloadVideoWithProgress(
  url: string,
  filename: string,
  onProgress?: (progress: number) => void
): Promise<void> {
  const response = await fetch(url)
  
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`)
  }
  
  const contentLength = response.headers.get('content-length')
  const total = contentLength ? parseInt(contentLength, 10) : 0
  
  if (!response.body) {
    throw new Error('Response body is null')
  }
  
  const reader = response.body.getReader()
  const chunks: Uint8Array[] = []
  let received = 0
  
  while (true) {
    const { done, value } = await reader.read()
    
    if (done) break
    
    chunks.push(value)
    received += value.length
    
    if (onProgress && total) {
      const progress = (received / total) * 100
      onProgress(progress)
    }
  }
  
  // 合并所有chunks
  const blob = new Blob(chunks)
  const objectUrl = URL.createObjectURL(blob)
  
  const link = document.createElement('a')
  link.href = objectUrl
  link.download = filename
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  
  setTimeout(() => URL.revokeObjectURL(objectUrl), 100)
}

/**
 * 获取文件扩展名
 * @param url URL或文件名
 */
export function getFileExtension(url: string): string {
  try {
    const urlObj = new URL(url)
    const pathname = urlObj.pathname
    const parts = pathname.split('.')
    return parts.length > 1 ? parts[parts.length - 1] : 'mp4'
  } catch {
    // 如果不是有效的URL，直接从字符串提取
    const parts = url.split('.')
    return parts.length > 1 ? parts[parts.length - 1] : 'mp4'
  }
}

/**
 * 生成安全的文件名
 * @param title 标题
 * @param extension 扩展名
 */
export function generateSafeFilename(title: string, extension: string = 'mp4'): string {
  // 移除不安全的字符
  const safe = title
    .replace(/[<>:"/\\|?*]/g, '')
    .replace(/\s+/g, '_')
    .substring(0, 100) // 限制长度
  
  return `${safe || 'video'}.${extension}`
}

