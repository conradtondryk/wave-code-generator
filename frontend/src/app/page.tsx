'use client'

import { useState } from 'react'
import { Music, Download, Loader2, Copy, ChevronRight, Sparkles } from 'lucide-react'

export default function Home() {
  const [playlistUrl, setPlaylistUrl] = useState('')
  const [title, setTitle] = useState('Spotify Wave Codes')
  const [columns, setColumns] = useState(4)
  const [imageSize, setImageSize] = useState(640)
  const [backgroundColor, setBackgroundColor] = useState('#ffffff')
  const [isGenerating, setIsGenerating] = useState(false)
  const [isExtracting, setIsExtracting] = useState(false)
  const [trackIds, setTrackIds] = useState<string[]>([])
  const [generatedHtml, setGeneratedHtml] = useState('')

  const handleExtractTracks = async () => {
    if (!playlistUrl) return

    setIsExtracting(true)
    try {
      const response = await fetch('/api/extract-tracks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ playlistUrl })
      })

      const data = await response.json()
      if (data.success) {
        setTrackIds(data.trackIds)
      } else {
        alert('Error: ' + data.error)
      }
    } catch (error) {
      alert('Failed to extract tracks: ' + error)
    } finally {
      setIsExtracting(false)
    }
  }

  const handleGenerateHtml = async () => {
    if (trackIds.length === 0) return

    setIsGenerating(true)
    try {
      const response = await fetch('/api/generate-html', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          trackIds,
          title,
          columns,
          imageSize,
          backgroundColor
        })
      })

      const data = await response.json()
      if (data.success) {
        setGeneratedHtml(data.html)
      } else {
        alert('Error: ' + data.error)
      }
    } catch (error) {
      alert('Failed to generate HTML: ' + error)
    } finally {
      setIsGenerating(false)
    }
  }

  const handleDownloadHtml = () => {
    if (!generatedHtml) return

    const blob = new Blob([generatedHtml], { type: 'text/html' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `${title.replace(/[^a-z0-9]/gi, '_').toLowerCase()}.html`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  const handleCopyTrackIds = () => {
    navigator.clipboard.writeText(trackIds.join('\n'))
  }

  return (
    <div className="min-h-screen bg-neutral-50">
      {/* Header */}
      <header className="bg-white border-b border-neutral-200">
        <div className="max-w-6xl mx-auto px-6 h-16 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-9 h-9 bg-neutral-900 rounded-lg flex items-center justify-center">
              <Music className="w-5 h-5 text-white" />
            </div>
            <h1 className="text-lg font-medium text-neutral-900">Wave Code Generator</h1>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-xs text-neutral-500 font-medium">POWERED BY</span>
            <span className="text-xs font-mono bg-neutral-100 text-neutral-700 px-2 py-1 rounded">RUST</span>
          </div>
        </div>
      </header>

      <main className="max-w-6xl mx-auto px-6 py-12">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Main Content */}
          <div className="lg:col-span-2 space-y-8">
            {/* Step 1: Input */}
            <div className="bg-white rounded-2xl border border-neutral-200 overflow-hidden">
              <div className="px-6 py-4 border-b border-neutral-100">
                <div className="flex items-center gap-2">
                  <span className="text-xs font-mono text-neutral-400">01</span>
                  <h2 className="text-sm font-medium text-neutral-900">Input Playlist</h2>
                </div>
              </div>

              <div className="p-6 space-y-4">
                <div>
                  <label htmlFor="playlist-url" className="block text-xs font-medium text-neutral-600 mb-2">
                    SPOTIFY PLAYLIST URL
                  </label>
                  <div className="relative">
                    <input
                      id="playlist-url"
                      type="url"
                      value={playlistUrl}
                      onChange={(e) => setPlaylistUrl(e.target.value)}
                      placeholder="https://open.spotify.com/playlist/..."
                      className="w-full px-4 py-3 bg-neutral-50 border border-neutral-200 rounded-lg text-sm text-neutral-900 placeholder-neutral-400 focus:outline-none focus:ring-2 focus:ring-neutral-900 focus:border-transparent transition-all"
                    />
                  </div>
                </div>

                <button
                  onClick={handleExtractTracks}
                  disabled={!playlistUrl || isExtracting}
                  className="w-full bg-neutral-900 text-white py-3 px-4 rounded-lg text-sm font-medium hover:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 flex items-center justify-center gap-2"
                >
                  {isExtracting ? (
                    <>
                      <Loader2 className="w-4 h-4 animate-spin" />
                      <span>Extracting...</span>
                    </>
                  ) : (
                    <>
                      <ChevronRight className="w-4 h-4" />
                      <span>Extract Tracks</span>
                    </>
                  )}
                </button>

                {trackIds.length > 0 && (
                  <div className="bg-neutral-50 rounded-lg p-4 flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <div className="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center">
                        <Sparkles className="w-4 h-4 text-green-600" />
                      </div>
                      <div>
                        <p className="text-sm font-medium text-neutral-900">{trackIds.length} tracks found</p>
                        <p className="text-xs text-neutral-500">Ready to generate</p>
                      </div>
                    </div>
                    <button
                      onClick={handleCopyTrackIds}
                      className="text-xs text-neutral-600 hover:text-neutral-900 flex items-center gap-1"
                    >
                      <Copy className="w-3 h-3" />
                      Copy IDs
                    </button>
                  </div>
                )}
              </div>
            </div>

            {/* Step 2: Preview */}
            {generatedHtml && (
              <div className="bg-white rounded-2xl border border-neutral-200 overflow-hidden">
                <div className="px-6 py-4 border-b border-neutral-100 flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <span className="text-xs font-mono text-neutral-400">03</span>
                    <h2 className="text-sm font-medium text-neutral-900">Preview</h2>
                  </div>
                  <button
                    onClick={handleDownloadHtml}
                    className="inline-flex items-center gap-2 bg-neutral-900 text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-neutral-800 transition-colors"
                  >
                    <Download className="w-4 h-4" />
                    <span>Download</span>
                  </button>
                </div>

                <div className="p-6">
                  <div className="bg-neutral-50 rounded-lg overflow-hidden border border-neutral-200">
                    <iframe
                      srcDoc={generatedHtml}
                      className="w-full h-96"
                      title="Wave codes preview"
                    />
                  </div>
                </div>
              </div>
            )}
          </div>

          {/* Sidebar - Settings */}
          <div className="space-y-6">
            {/* Configuration */}
            <div className="bg-white rounded-2xl border border-neutral-200 overflow-hidden">
              <div className="px-6 py-4 border-b border-neutral-100">
                <div className="flex items-center gap-2">
                  <span className="text-xs font-mono text-neutral-400">02</span>
                  <h2 className="text-sm font-medium text-neutral-900">Configuration</h2>
                </div>
              </div>

              <div className="p-6 space-y-6">
                {/* Title */}
                <div>
                  <label htmlFor="title" className="block text-xs font-medium text-neutral-600 mb-2">
                    PAGE TITLE
                  </label>
                  <input
                    id="title"
                    type="text"
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                    className="w-full px-3 py-2 bg-neutral-50 border border-neutral-200 rounded-lg text-sm text-neutral-900 focus:outline-none focus:ring-2 focus:ring-neutral-900 focus:border-transparent transition-all"
                  />
                </div>

                {/* Columns */}
                <div>
                  <label htmlFor="columns" className="block text-xs font-medium text-neutral-600 mb-2">
                    COLUMNS <span className="font-mono text-neutral-400 ml-1">{columns}</span>
                  </label>
                  <input
                    id="columns"
                    type="range"
                    min="1"
                    max="8"
                    value={columns}
                    onChange={(e) => setColumns(Number(e.target.value))}
                    className="w-full h-1.5 bg-neutral-200 rounded-full appearance-none cursor-pointer slider-minimal"
                  />
                  <div className="flex justify-between text-[10px] text-neutral-400 mt-2">
                    <span>1</span>
                    <span>4</span>
                    <span>8</span>
                  </div>
                </div>

                {/* Image Size */}
                <div>
                  <label htmlFor="image-size" className="block text-xs font-medium text-neutral-600 mb-2">
                    IMAGE SIZE <span className="font-mono text-neutral-400 ml-1">{imageSize}px</span>
                  </label>
                  <input
                    id="image-size"
                    type="range"
                    min="320"
                    max="1280"
                    step="80"
                    value={imageSize}
                    onChange={(e) => setImageSize(Number(e.target.value))}
                    className="w-full h-1.5 bg-neutral-200 rounded-full appearance-none cursor-pointer slider-minimal"
                  />
                  <div className="flex justify-between text-[10px] text-neutral-400 mt-2">
                    <span>320</span>
                    <span>800</span>
                    <span>1280</span>
                  </div>
                </div>

                {/* Background Color */}
                <div>
                  <label htmlFor="bg-color" className="block text-xs font-medium text-neutral-600 mb-2">
                    BACKGROUND
                  </label>
                  <div className="flex gap-2">
                    <input
                      id="bg-color"
                      type="color"
                      value={backgroundColor}
                      onChange={(e) => setBackgroundColor(e.target.value)}
                      className="w-10 h-10 bg-neutral-50 border border-neutral-200 rounded-lg cursor-pointer"
                    />
                    <input
                      type="text"
                      value={backgroundColor}
                      onChange={(e) => setBackgroundColor(e.target.value)}
                      className="flex-1 px-3 py-2 bg-neutral-50 border border-neutral-200 rounded-lg text-sm font-mono text-neutral-700 uppercase focus:outline-none focus:ring-2 focus:ring-neutral-900 focus:border-transparent"
                    />
                  </div>

                  <div className="grid grid-cols-6 gap-2 mt-3">
                    {['#ffffff', '#fafafa', '#f5f5f5', '#e5e5e5', '#171717', '#000000'].map((color) => (
                      <button
                        key={color}
                        onClick={() => setBackgroundColor(color)}
                        className={`w-full h-8 rounded-lg border-2 transition-all ${
                          backgroundColor === color ? 'border-neutral-900' : 'border-neutral-200'
                        }`}
                        style={{ backgroundColor: color }}
                        title={color}
                      />
                    ))}
                  </div>
                </div>

                {/* Generate Button */}
                <div className="pt-2">
                  <button
                    onClick={handleGenerateHtml}
                    disabled={trackIds.length === 0 || isGenerating}
                    className="w-full bg-neutral-900 text-white py-3.5 px-4 rounded-lg font-medium hover:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 flex items-center justify-center gap-2"
                  >
                    {isGenerating ? (
                      <>
                        <Loader2 className="w-4 h-4 animate-spin" />
                        <span>Generating...</span>
                      </>
                    ) : (
                      <>
                        <Sparkles className="w-4 h-4" />
                        <span>Generate Wave Codes</span>
                      </>
                    )}
                  </button>
                </div>
              </div>
            </div>

            {/* Info */}
            <div className="text-xs text-neutral-500 px-2 space-y-1">
              <p>Generate printable Spotify wave codes</p>
              <p className="font-mono text-[10px]">Rust + Next.js</p>
            </div>
          </div>
        </div>
      </main>
    </div>
  )
}