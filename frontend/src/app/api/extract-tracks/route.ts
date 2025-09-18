import { NextRequest, NextResponse } from 'next/server'
import { spawn } from 'child_process'
import { readFileSync, unlinkSync } from 'fs'
import { join } from 'path'

export async function POST(request: NextRequest) {
    try {
        const { playlistUrl } = await request.json()

        if (!playlistUrl) {
            return NextResponse.json({ success: false, error: 'Playlist URL is required' })
        }

        // Get Spotify credentials from environment variables
        const clientId = process.env.SPOTIFY_CLIENT_ID
        const clientSecret = process.env.SPOTIFY_CLIENT_SECRET

        if (!clientId || !clientSecret) {
            return NextResponse.json({
                success: false,
                error: 'Spotify API credentials not configured. Please set SPOTIFY_CLIENT_ID and SPOTIFY_CLIENT_SECRET environment variables.'
            })
        }

        // Generate a unique filename for this request
        const timestamp = Date.now()
        const outputFile = `playlist_${timestamp}.txt`

        return new Promise<NextResponse>((resolve) => {
            // Execute the Rust binary to extract track IDs
            const rustBinary = join(process.cwd(), '..', 'target', 'debug', 'get-song-ids')
            const child = spawn(rustBinary, [
                '--url', playlistUrl,
                '--client-id', clientId,
                '--client-secret', clientSecret,
                '--output', outputFile
            ], {
                cwd: join(process.cwd(), '..')
            })

            let stdout = ''
            let stderr = ''

            child.stdout.on('data', (data) => {
                stdout += data.toString()
            })

            child.stderr.on('data', (data) => {
                stderr += data.toString()
            })

            child.on('close', (code) => {
                if (code === 0) {
                    try {
                        // Read the generated track IDs file
                        const inputFilePath = join(process.cwd(), '..', 'input', outputFile)
                        const trackIdsContent = readFileSync(inputFilePath, 'utf-8')
                        const trackIds = trackIdsContent.trim().split('\\n').filter(id => id.length > 0)

                        // Clean up the temporary file
                        unlinkSync(inputFilePath)

                        resolve(NextResponse.json({
                            success: true,
                            trackIds,
                            message: `Successfully extracted ${trackIds.length} track IDs`
                        }))
                    } catch (error) {
                        resolve(NextResponse.json({
                            success: false,
                            error: `Failed to read track IDs: ${error}`
                        }))
                    }
                } else {
                    resolve(NextResponse.json({
                        success: false,
                        error: `Failed to extract tracks: ${stderr || stdout}`
                    }))
                }
            })

            child.on('error', (error) => {
                resolve(NextResponse.json({
                    success: false,
                    error: `Failed to execute track extractor: ${error.message}`
                }))
            })
        })
    } catch (error) {
        return NextResponse.json({
            success: false,
            error: `Server error: ${error}`
        })
    }
}
