'use client'

import { useState } from 'react'
import { Music, Settings, Download, Palette, Grid, Maximize } from 'lucide-react'

export default function Home() {
  const [playlistUrl, setPlaylistUrl] = useState('')
  const [title, setTitle] = useState('My Spotify Codes')
  const [columns, setColumns] = useState(4)
  const [imageSize, setImageSize] = useState(640)
  const [backgroundColor, setBackgroundColor] = useState('#ffffff')
  const [isGenerating, setIsGenerating] = useState(false)
  const [trackIds, setTrackIds] = useState<string[]>([])
  const [generatedHtml, setGeneratedHtml] = useState('')

  const handleExtractTracks = async () => {
    if (!playlistUrl) return

    setIsGenerating(true)
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
      setIsGenerating(false)
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

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-50 to-blue-50">
      {/* Header */}
      <header className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center space-x-3">
            <div className="bg-gradient-to-r from-purple-500 to-blue-500 p-2 rounded-lg">
              <Music className="h-6 w-6 text-white" />
            </div>
            <h1 className="text-2xl font-bold text-gray-900">Wave Code Generator</h1>
            <span className="text-sm text-gray-500 bg-gray-100 px-2 py-1 rounded-full">Rust + Next.js</span>
          </div>
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Input Section */}
          <div className="lg:col-span-2 space-y-6">
            {/* Playlist Input */}
            <div className="bg-white rounded-xl shadow-sm p-6 border">
              <div className="flex items-center space-x-2 mb-4">
                <Music className="h-5 w-5 text-purple-500" />
                <h2 className="text-lg font-semibold text-gray-900">Spotify Playlist</h2>
              </div>

              <div className="space-y-4">
                <div>
                  <label htmlFor="playlist-url" className="block text-sm font-medium text-gray-700 mb-2">
                    Playlist URL
                  </label>
                  <input
                    id="playlist-url"
                    type="url"
                    value={playlistUrl}
                    onChange={(e) => setPlaylistUrl(e.target.value)}
                    placeholder="https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M"
                    className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                  />
                </div>

                <button
                  onClick={handleExtractTracks}
                  disabled={!playlistUrl || isGenerating}
                  className="w-full bg-gradient-to-r from-purple-500 to-blue-500 text-white py-3 px-4 rounded-lg font-medium hover:from-purple-600 hover:to-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200"
                >
                  {isGenerating ? 'Extracting Tracks...' : 'Extract Track IDs'}
                </button>

                {trackIds.length > 0 && (
                  <div className="bg-green-50 border border-green-200 rounded-lg p-4">
                    <p className="text-green-800 font-medium">✅ Found {trackIds.length} tracks</p>
                    <p className="text-green-600 text-sm mt-1">Ready to generate wave codes!</p>
                  </div>
                )}
              </div>
            </div>

            {/* Preview Section */}
            {generatedHtml && (
              <div className="bg-white rounded-xl shadow-sm p-6 border">
                <div className="flex items-center justify-between mb-4">
                  <h2 className="text-lg font-semibold text-gray-900">Preview</h2>
                  <button
                    onClick={handleDownloadHtml}
                    className="inline-flex items-center space-x-2 bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-600 transition-colors"
                  >
                    <Download className="h-4 w-4" />
                    <span>Download HTML</span>
                  </button>
                </div>

                <div className="border rounded-lg overflow-hidden">
                  <iframe
                    srcDoc={generatedHtml}
                    className="w-full h-96 border-0"
                    title="Wave codes preview"
                  />
                </div>
              </div>
            )}
          </div>

          {/* Settings Sidebar */}
          <div className="space-y-6">
            {/* Page Settings */}
            <div className="bg-white rounded-xl shadow-sm p-6 border">
              <div className="flex items-center space-x-2 mb-4">
                <Settings className="h-5 w-5 text-blue-500" />
                <h2 className="text-lg font-semibold text-gray-900">Page Settings</h2>
              </div>

              <div className="space-y-4">
                <div>
                  <label htmlFor="title" className="block text-sm font-medium text-gray-700 mb-2">
                    Page Title
                  </label>
                  <input
                    id="title"
                    type="text"
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  />
                </div>
              </div>
            </div>

            {/* Layout Settings */}
            <div className="bg-white rounded-xl shadow-sm p-6 border">
              <div className="flex items-center space-x-2 mb-4">
                <Grid className="h-5 w-5 text-green-500" />
                <h2 className="text-lg font-semibold text-gray-900">Layout</h2>
              </div>

              <div className="space-y-4">
                <div>
                  <label htmlFor="columns" className="block text-sm font-medium text-gray-700 mb-2">
                    Columns: {columns}
                  </label>
                  <input
                    id="columns"
                    type="range"
                    min="1"
                    max="8"
                    value={columns}
                    onChange={(e) => setColumns(Number(e.target.value))}
                    className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer slider"
                  />
                  <div className="flex justify-between text-xs text-gray-500 mt-1">
                    <span>1</span>
                    <span>4</span>
                    <span>8</span>
                  </div>
                </div>

                <div>
                  <label htmlFor="image-size" className="block text-sm font-medium text-gray-700 mb-2">
                    Image Size: {imageSize}px
                  </label>
                  <input
                    id="image-size"
                    type="range"
                    min="320"
                    max="1280"
                    step="80"
                    value={imageSize}
                    onChange={(e) => setImageSize(Number(e.target.value))}
                    className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer slider"
                  />
                  <div className="flex justify-between text-xs text-gray-500 mt-1">
                    <span>320px</span>
                    <span>640px</span>
                    <span>1280px</span>
                  </div>
                </div>
              </div>
            </div>

            {/* Style Settings */}
            <div className="bg-white rounded-xl shadow-sm p-6 border">
              <div className="flex items-center space-x-2 mb-4">
                <Palette className="h-5 w-5 text-pink-500" />
                <h2 className="text-lg font-semibold text-gray-900">Style</h2>
              </div>

              <div className="space-y-4">
                <div>
                  <label htmlFor="bg-color" className="block text-sm font-medium text-gray-700 mb-2">
                    Background Color
                  </label>
                  <div className="flex space-x-2">
                    <input
                      id="bg-color"
                      type="color"
                      value={backgroundColor}
                      onChange={(e) => setBackgroundColor(e.target.value)}
                      className="w-12 h-10 border border-gray-300 rounded cursor-pointer"
                    />
                    <input
                      type="text"
                      value={backgroundColor}
                      onChange={(e) => setBackgroundColor(e.target.value)}
                      className="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-pink-500 focus:border-transparent font-mono text-sm"
                    />
                  </div>
                </div>

                <div className="grid grid-cols-4 gap-2">
                  {['#ffffff', '#f8f9fa', '#e9ecef', '#000000'].map((color) => (
                    <button
                      key={color}
                      onClick={() => setBackgroundColor(color)}
                      className={`w-full h-8 rounded border-2 transition-all ${backgroundColor === color ? 'border-gray-900' : 'border-gray-300'
                        }`}
                      style={{ backgroundColor: color }}
                      title={color}
                    />
                  ))}
                </div>
              </div>
            </div>

            {/* Generate Button */}
            <button
              onClick={handleGenerateHtml}
              disabled={trackIds.length === 0 || isGenerating}
              className="w-full bg-gradient-to-r from-green-500 to-teal-500 text-white py-4 px-6 rounded-xl font-semibold hover:from-green-600 hover:to-teal-600 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 shadow-lg"
            >
              {isGenerating ? (
                <div className="flex items-center justify-center space-x-2">
                  <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                  <span>Generating...</span>
                </div>
              ) : (
                <div className="flex items-center justify-center space-x-2">
                  <Maximize className="h-5 w-5" />
                  <span>Generate Wave Codes</span>
                </div>
              )}
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}